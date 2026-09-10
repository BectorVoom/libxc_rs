//! libxc's `xc_math_brent` (`math_brent.c`), transcribed.
//!
//! Brent's method: inverse quadratic interpolation where it behaves, secant
//! where it does not, bisection whenever either would step outside the
//! bracket or fail to shrink it fast enough.
//!
//! **The stopping rule is part of the answer.** Brent does not stand still
//! once it is inside the tolerance -- it keeps interpolating and bisecting
//! within the bracket -- so a fixed number of iterations returns a different
//! point than libxc's "return `(a+b)/2` as soon as `|b-a| < TOL`". Both are
//! roots to within `TOL`, and they differ in the last few digits, which is
//! exactly the size of the residual `mgga_x_br89` and friends carried against
//! the oracle.
//!
//! The callers (`br89`, `mbrxc`) used to each carry their own copy of the
//! iteration, unrolled 60 times with branchless `select` updates, because
//! CubeCL `#[cube]` kernels had no dynamic loops or function pointers. CubeCL
//! is gone; there is one loop again, as there is in libxc.

/// Solve `f(x) = 0` on `[lower_bound, upper_bound]`.
///
/// libxc prints a bracketing diagnostic and calls `exit(1)` when
/// `f(a)*f(b) > 0`; a library has no business terminating the host process,
/// so that case falls through into the iteration here -- which is what
/// libxc's own CUDA build does, since the `exit` is inside `#ifndef
/// HAVE_CUDA`. Both callers construct a bracket that is valid by
/// construction.
///
/// Likewise libxc warns on stderr when the iteration limit is reached and
/// returns the midpoint anyway; this returns the midpoint without the warning.
pub fn xc_math_brent<F>(f: F, lower_bound: f64, upper_bound: f64, tol: f64, max_iter: i32) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut a = lower_bound;
    let mut b = upper_bound;
    let mut fa = f(a);
    let mut fb = f(b);

    if fa.abs() < fb.abs() {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut fa, &mut fb);
    }

    // `c` now holds the larger-magnitude end; `d` is only read once `mflag`
    // has been unset at least once, exactly as in the C.
    let mut c = a;
    let mut fc = fa;
    let mut mflag = true;
    let mut d = 0.0f64;

    let mut iter = 1;
    while iter < max_iter {
        // Converged, or the bracket is below tolerance.
        if (b - a).abs() < tol {
            return (b + a) / 2.0;
        }

        let mut s = if fa != fc && fb != fc {
            // inverse quadratic interpolation
            (a * fb * fc / ((fa - fb) * (fa - fc)))
                + (b * fa * fc / ((fb - fa) * (fb - fc)))
                + (c * fa * fb / ((fc - fa) * (fc - fb)))
        } else {
            // secant method
            b - fb * (b - a) / (fb - fa)
        };

        // (1) s is not between (3a+b)/4 and b, or
        // (2) mflag and |s-b| >= |b-c|/2, or
        // (3) !mflag and |s-b| >= |c-d|/2, or
        // (4) mflag and |b-c| < TOL, or
        // (5) !mflag and |c-d| < TOL.
        if ((s < (3.0 * a + b) * 0.25) || (s > b))
            || (mflag && ((s - b).abs() >= ((b - c).abs() * 0.5)))
            || (!mflag && ((s - b).abs() >= ((c - d).abs() * 0.5)))
            || (mflag && ((b - c).abs() < tol))
            || (!mflag && ((c - d).abs() < tol))
        {
            s = (a + b) * 0.5;
            mflag = true;
        } else {
            mflag = false;
        }

        let fs = f(s);
        d = c;
        c = b;
        fc = fb;

        if fa * fs < 0.0 {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }

        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }

        iter += 1;
    }

    (b + a) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A bracket already inside `tol` returns its midpoint without evaluating
    /// anything past the initial two points -- the early return, not the loop
    /// bound, is what ends the call.
    #[test]
    fn converged_bracket_returns_midpoint() {
        let x = xc_math_brent(|x| x - 1.0, 1.0 - 1e-15, 1.0 + 1e-15, 5e-12, 500);
        assert!((x - 1.0).abs() < 1e-14, "got {x}");
    }

    /// A plain cubic, to show the iteration converges rather than running out
    /// of budget.
    #[test]
    fn finds_a_simple_root() {
        let x = xc_math_brent(|x| x * x * x - 2.0, 0.0, 2.0, 5e-12, 500);
        assert!((x - 2f64.cbrt()).abs() < 1e-10, "got {x}");
    }
}
