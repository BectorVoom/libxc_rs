//! QUADPACK `dqagse`, transcribed from the copy libxc ships.
//!
//! # Why a transcription and not an equivalent rule
//!
//! `xc_integrate` in `libxc-master/src/integrate.c` is QUADPACK's `dqagse`:
//! adaptive bisection driven by a 21-point Gauss–Kronrod rule, with Wynn's
//! epsilon algorithm extrapolating the sequence of partial results, run to
//! `epsabs = epsrel = 1e-10` over at most 1000 subintervals.
//!
//! That tolerance is the whole problem. libxc's own answer is only accurate to
//! ~1e-10, so a *different* quadrature — however accurate — disagrees with it
//! at that level and cannot meet this project's 1e-12 parity contract. The
//! composite Gauss–Legendre in `integrate.rs` was accurate to ~1e-12 of the
//! true integral and still missed libxc by 7.8e-8 on `lda_x_1d_exponential`.
//! Matching requires making the *same* approximation, node for node.
//!
//! So this is a literal transcription: same rule, same weights, same bisection
//! order, same extrapolation, same convergence tests. Arrays are 1-based (index
//! 0 unused) to keep the correspondence with the Fortran-derived C obvious --
//! the C does `--iord; --elist;` on entry for exactly that reason. Only
//! `result` is returned, which is all `xc_integrate` uses.

/// Nodes of the 21-point Kronrod rule on `[-1, 1]`, positive half.
/// `xgk[1], xgk[3], …` are the 10-point Gauss nodes.
const XGK: [f64; 11] = [
    0.995657163025808080735527280689003,
    0.973906528517171720077964012084452,
    0.930157491355708226001207180059508,
    0.865063366688984510732096688423493,
    0.780817726586416897063717578345042,
    0.679409568299024406234327365114874,
    0.562757134668604683339000099272694,
    0.433395394129247190799265943165784,
    0.294392862701460198131126603103866,
    0.14887433898163121088482600112972,
    0.0,
];

/// Weights of the 21-point Kronrod rule.
const WGK: [f64; 11] = [
    0.011694638867371874278064396062192,
    0.03255816230796472747881897245939,
    0.05475589657435199603138130024458,
    0.07503967481091995276704314091619,
    0.093125454583697605535065465083366,
    0.109387158802297641899210590325805,
    0.123491976262065851077958109831074,
    0.134709217311473325928054001771707,
    0.142775938577060080797094273138717,
    0.147739104901338491374841515972068,
    0.149445554002916905664936468389821,
];

/// Weights of the embedded 10-point Gauss rule.
const WG: [f64; 5] = [
    0.066671344308688137593568809893332,
    0.149451349150580593145776339657697,
    0.219086362515982043995534934228163,
    0.269266719309996355091226921569469,
    0.295524224714752870173892994651338,
];

/// One 21-point Gauss–Kronrod panel over `[a, b]`.
///
/// Returns `(result, abserr, resabs, resasc)`, matching `rdqk21`.
fn dqk21<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64) -> (f64, f64, f64, f64) {
    let epmach = f64::EPSILON;
    let uflow = f64::MIN_POSITIVE;

    let centr = (a + b) * 0.5;
    let hlgth = (b - a) * 0.5;
    let dhlgth = hlgth.abs();

    // The C fills a 21-slot vector and calls the integrand on all of it at
    // once; the layout below is that vector's, so the evaluation order (and
    // hence nothing at all, the calls being independent) is preserved.
    let fc = f(centr);
    let mut resk = WGK[10] * fc;
    let mut resabs = resk.abs();
    let mut resg = 0.0f64;
    let mut fv1 = [0.0f64; 10];
    let mut fv2 = [0.0f64; 10];

    for j in 1..=5 {
        let jtw = 2 * j;
        let absc = hlgth * XGK[jtw - 1];
        let fval1 = f(centr - absc);
        let fval2 = f(centr + absc);
        fv1[jtw - 1] = fval1;
        fv2[jtw - 1] = fval2;
        let fsum = fval1 + fval2;
        resg += WG[j - 1] * fsum;
        resk += WGK[jtw - 1] * fsum;
        resabs += WGK[jtw - 1] * (fval1.abs() + fval2.abs());
    }
    for j in 1..=5 {
        let jtwm1 = 2 * j - 1;
        let absc = hlgth * XGK[jtwm1 - 1];
        let fval1 = f(centr - absc);
        let fval2 = f(centr + absc);
        fv1[jtwm1 - 1] = fval1;
        fv2[jtwm1 - 1] = fval2;
        let fsum = fval1 + fval2;
        resk += WGK[jtwm1 - 1] * fsum;
        resabs += WGK[jtwm1 - 1] * (fval1.abs() + fval2.abs());
    }

    let reskh = resk * 0.5;
    let mut resasc = WGK[10] * (fc - reskh).abs();
    for j in 1..=10 {
        resasc += WGK[j - 1] * ((fv1[j - 1] - reskh).abs() + (fv2[j - 1] - reskh).abs());
    }

    let result = resk * hlgth;
    let resabs = resabs * dhlgth;
    let resasc = resasc * dhlgth;
    let mut abserr = ((resk - resg) * hlgth).abs();
    if resasc != 0.0 && abserr != 0.0 {
        abserr = resasc * 1.0f64.min((abserr * 200.0 / resasc).powf(1.5));
    }
    if resabs > uflow / (epmach * 50.0) {
        abserr = (epmach * 50.0 * resabs).max(abserr);
    }
    (result, abserr, resabs, resasc)
}

/// Wynn's epsilon algorithm (`rdqelg`).
///
/// `epstab` is 1-based with 52 usable slots (the C indexes `epstab[n + 2]`).
/// Mutates `n`, `epstab`, `res3la` and `nres` exactly as the C does.
fn dqelg(
    n: &mut usize,
    epstab: &mut [f64; 55],
    res3la: &mut [f64; 4],
    nres: &mut usize,
) -> (f64, f64) {
    let epmach = f64::EPSILON;
    let oflow = f64::MAX;
    *nres += 1;
    let mut abserr = oflow;
    let mut result = epstab[*n];
    if *n < 3 {
        return (result, abserr.max(epmach * 5.0 * result.abs()));
    }
    let limexp = 50usize;
    epstab[*n + 2] = epstab[*n];
    let newelm = (*n - 1) / 2;
    epstab[*n] = oflow;
    let num = *n;
    let mut k1 = *n;
    let mut res = 0.0f64;
    let mut jumped_out = false;

    for i in 1..=newelm {
        let k2 = k1 - 1;
        let k3 = k1 - 2;
        res = epstab[k1 + 2];
        let e0 = epstab[k3];
        let e1 = epstab[k2];
        let e2 = res;
        let e1abs = e1.abs();
        let delta2 = e2 - e1;
        let err2 = delta2.abs();
        let tol2 = e2.abs().max(e1abs) * epmach;
        let delta3 = e1 - e0;
        let err3 = delta3.abs();
        let tol3 = e1abs.max(e0.abs()) * epmach;
        if err2 <= tol2 && err3 <= tol3 {
            // e0, e1, e2 agree to machine accuracy: assume convergence.
            result = res;
            abserr = err2 + err3;
            return (result, abserr.max(epmach * 5.0 * result.abs()));
        }
        let e3 = epstab[k1];
        epstab[k1] = e1;
        let delta1 = e1 - e3;
        let err1 = delta1.abs();
        let tol1 = e1abs.max(e3.abs()) * epmach;

        let mut ss = 0.0f64;
        let mut ok = false;
        if err1 > tol1 && err2 > tol2 && err3 > tol3 {
            ss = 1.0 / delta1 + 1.0 / delta2 - 1.0 / delta3;
            if (ss * e1).abs() > 1e-4 {
                ok = true;
            }
        }
        if !ok {
            // Irregular behaviour: drop part of the table.
            *n = i + i - 1;
            jumped_out = true;
            break;
        }
        res = e1 + 1.0 / ss;
        epstab[k1] = res;
        k1 -= 2;
        let err_a = err2 + (res - e2).abs() + err3;
        if err_a <= abserr {
            abserr = err_a;
            result = res;
        }
    }
    let _ = jumped_out;

    // Shift the table.
    if *n == limexp {
        *n = (limexp / 2) * 2 - 1;
    }
    let mut ib = if (num / 2) * 2 == num { 2 } else { 1 };
    let ie = newelm + 1;
    for _ in 1..=ie {
        let ib2 = ib + 2;
        epstab[ib] = epstab[ib2];
        ib = ib2;
    }
    if num != *n {
        let mut indx = num - *n + 1;
        for i in 1..=*n {
            epstab[i] = epstab[indx];
            indx += 1;
        }
    }
    if *nres >= 4 {
        abserr = (result - res3la[3]).abs()
            + (result - res3la[2]).abs()
            + (result - res3la[1]).abs();
        res3la[1] = res3la[2];
        res3la[2] = res3la[3];
        res3la[3] = result;
    } else {
        res3la[*nres] = result;
        abserr = oflow;
    }
    (result, abserr.max(epmach * 5.0 * result.abs()))
}

/// Maintain the descending ordering of the error list (`rdqpsrt`).
fn dqpsrt(
    limit: usize,
    last: usize,
    maxerr: &mut usize,
    ermax: &mut f64,
    elist: &[f64],
    iord: &mut [usize],
    nrmax: &mut usize,
) {
    if last <= 2 {
        iord[1] = 1;
        iord[2] = 2;
        *maxerr = iord[*nrmax];
        *ermax = elist[*maxerr];
        return;
    }
    let errmax = elist[*maxerr];
    if *nrmax > 1 {
        let ido = *nrmax - 1;
        for _ in 1..=ido {
            let isucc = iord[*nrmax - 1];
            if errmax <= elist[isucc] {
                break;
            }
            iord[*nrmax] = isucc;
            *nrmax -= 1;
        }
    }
    let jupbn = if last > limit / 2 + 2 { limit + 3 - last } else { last };
    let errmin = elist[last];
    let jbnd = jupbn - 1;

    let mut placed = false;
    let mut i = *nrmax + 1;
    while i <= jbnd {
        let isucc = iord[i];
        if errmax >= elist[isucc] {
            iord[i - 1] = *maxerr;
            // Insert errmin bottom-up.
            let mut k = jbnd;
            let mut done = false;
            for _ in i..=jbnd {
                let isucc2 = iord[k];
                if errmin < elist[isucc2] {
                    iord[k + 1] = last;
                    done = true;
                    break;
                }
                iord[k + 1] = isucc2;
                k -= 1;
            }
            if !done {
                iord[i] = last;
            }
            placed = true;
            break;
        }
        iord[i - 1] = isucc;
        i += 1;
    }
    if !placed {
        iord[jbnd] = *maxerr;
        iord[jupbn] = last;
    }
    *maxerr = iord[*nrmax];
    *ermax = elist[*maxerr];
}

/// `xc_integrate(f, NULL, a, b)`: QUADPACK `dqagse` with libxc's settings.
///
/// libxc passes `epsabs = epsrel = 1e-10` and `limit = 1000`, and keeps only
/// `result`.
pub fn xc_integrate<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64) -> f64 {
    const EPSABS: f64 = 1e-10;
    const EPSREL: f64 = 1e-10;
    const LIMIT: usize = 1000;

    let epmach = f64::EPSILON;
    let uflow = f64::MIN_POSITIVE;
    let oflow = f64::MAX;

    let mut alist = vec![0.0f64; LIMIT + 2];
    let mut blist = vec![0.0f64; LIMIT + 2];
    let mut rlist = vec![0.0f64; LIMIT + 2];
    let mut elist = vec![0.0f64; LIMIT + 2];
    let mut iord = vec![0usize; LIMIT + 2];
    let mut rlist2 = [0.0f64; 55];
    let mut res3la = [0.0f64; 4];

    let mut ier = 0i32;
    let mut last = 0usize;
    alist[1] = a;
    blist[1] = b;

    let (mut result, mut abserr, defabs, resabs0) = dqk21(f, a, b);

    let dres = result.abs();
    let mut errbnd = EPSABS.max(EPSREL * dres);
    last = 1;
    rlist[1] = result;
    elist[1] = abserr;
    iord[1] = 1;
    if abserr <= epmach * 100.0 * defabs && abserr > errbnd {
        ier = 2;
    }
    if ier != 0 || (abserr <= errbnd && abserr != resabs0) || abserr == 0.0 {
        return result;
    }

    rlist2[1] = result;
    let mut errmax = abserr;
    let mut maxerr = 1usize;
    let mut area = result;
    let mut errsum = abserr;
    abserr = oflow;
    let mut nrmax = 1usize;
    let mut nres = 0usize;
    let mut numrl2 = 2usize;
    let mut ktmin = 0usize;
    let mut extrap = false;
    let mut noext = false;
    let (mut iroff1, mut iroff2, mut iroff3) = (0i32, 0i32, 0i32);
    let mut ierro = 0i32;
    let ksgn: i32 = if dres >= (1.0 - epmach * 50.0) * defabs { 1 } else { -1 };

    let (mut correc, mut erlarg, mut ertest, mut small) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    // `L115` in the C: recompute the result as the plain sum of the panels.
    let mut sum_fallback = false;

    'main: for l in 2..=LIMIT {
        last = l;
        let a1 = alist[maxerr];
        let b1 = (alist[maxerr] + blist[maxerr]) * 0.5;
        let a2 = b1;
        let b2 = blist[maxerr];
        let erlast = errmax;
        let (area1, error1, _, defab1) = dqk21(f, a1, b1);
        let (area2, error2, _, defab2) = dqk21(f, a2, b2);

        let area12 = area1 + area2;
        let erro12 = error1 + error2;
        errsum = errsum + erro12 - errmax;
        area = area + area12 - rlist[maxerr];
        if defab1 != error1 && defab2 != error2 {
            if !((rlist[maxerr] - area12).abs() > area12.abs() * 1e-5 || erro12 >= errmax * 0.99) {
                if extrap {
                    iroff2 += 1;
                } else {
                    iroff1 += 1;
                }
            }
            if last > 10 && erro12 > errmax {
                iroff3 += 1;
            }
        }
        rlist[maxerr] = area1;
        rlist[last] = area2;
        errbnd = EPSABS.max(EPSREL * area.abs());

        if iroff1 + iroff2 >= 10 || iroff3 >= 20 {
            ier = 2;
        }
        if iroff2 >= 5 {
            ierro = 3;
        }
        if last == LIMIT {
            ier = 1;
        }
        if a1.abs().max(b2.abs()) <= (epmach * 100.0 + 1.0) * (a2.abs() + uflow * 1e3) {
            ier = 4;
        }

        if error2 > error1 {
            alist[maxerr] = a2;
            alist[last] = a1;
            blist[last] = b1;
            rlist[maxerr] = area2;
            rlist[last] = area1;
            elist[maxerr] = error2;
            elist[last] = error1;
        } else {
            alist[last] = a2;
            blist[maxerr] = b1;
            blist[last] = b2;
            elist[maxerr] = error1;
            elist[last] = error2;
        }

        dqpsrt(LIMIT, last, &mut maxerr, &mut errmax, &elist, &mut iord, &mut nrmax);

        if errsum <= errbnd {
            sum_fallback = true;
            break 'main;
        }
        if ier != 0 {
            break 'main;
        }
        if last == 2 {
            // L80
            small = (b - a).abs() * 0.375;
            erlarg = errsum;
            ertest = errbnd;
            rlist2[2] = area;
            continue 'main;
        }
        if noext {
            continue 'main;
        }

        erlarg -= erlast;
        if (b1 - a1).abs() > small {
            erlarg += erro12;
        }
        // L40 is entered directly when `extrap`; otherwise test the interval
        // about to be bisected first.
        if !extrap {
            if (blist[maxerr] - alist[maxerr]).abs() > small {
                continue 'main;
            }
            extrap = true;
            nrmax = 2;
        }

        if ierro != 3 && erlarg > ertest {
            // Shrink erlarg over the larger intervals before extrapolating.
            let id = nrmax;
            let jupbnd = if last > LIMIT / 2 + 2 { LIMIT + 3 - last } else { last };
            let mut goto90 = false;
            for _ in id..=jupbnd {
                maxerr = iord[nrmax];
                errmax = elist[maxerr];
                if (blist[maxerr] - alist[maxerr]).abs() > small {
                    goto90 = true;
                    break;
                }
                nrmax += 1;
            }
            if goto90 {
                continue 'main;
            }
        }

        // L60: extrapolate.
        numrl2 += 1;
        rlist2[numrl2] = area;
        let (reseps, abseps) = dqelg(&mut numrl2, &mut rlist2, &mut res3la, &mut nres);
        ktmin += 1;
        if ktmin > 5 && abserr < errsum * 0.001 {
            ier = 5;
        }
        if abseps < abserr {
            ktmin = 0;
            abserr = abseps;
            result = reseps;
            correc = erlarg;
            ertest = EPSABS.max(EPSREL * reseps.abs());
            if abserr <= ertest {
                break 'main;
            }
        }
        // L70: prepare bisection of the smallest interval.
        if numrl2 == 1 {
            noext = true;
        }
        if ier == 5 {
            break 'main;
        }
        maxerr = iord[1];
        errmax = elist[maxerr];
        nrmax = 1;
        extrap = false;
        small *= 0.5;
        erlarg = errsum;
    }

    // L100 onwards: settle the final result.
    if !sum_fallback {
        if abserr == oflow {
            sum_fallback = true;
        } else if ier + ierro != 0 {
            if ierro == 3 {
                abserr += correc;
            }
            if ier == 0 {
                ier = 3;
            }
            if result != 0.0 && area != 0.0 {
                if abserr / result.abs() > errsum / area.abs() {
                    sum_fallback = true;
                }
            } else if abserr > errsum {
                sum_fallback = true;
            } else if area == 0.0 {
                return result;
            }
        }
        if !sum_fallback
            && ksgn == -1
            && result.abs().max(area.abs()) <= defabs * 0.01
        {
            return result;
        }
    }

    if sum_fallback {
        result = 0.0;
        for k in 1..=last {
            result += rlist[k];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smooth integrand with a known closed form.
    #[test]
    fn integrates_a_polynomial_exactly() {
        // ∫₀² x³ dx = 4
        let got = xc_integrate(&|x: f64| x * x * x, 0.0, 2.0);
        assert!((got - 4.0).abs() < 1e-13, "got {got}");
    }

    /// The case the adaptive machinery exists for: an integrable endpoint
    /// singularity, which a fixed rule handles badly.
    #[test]
    fn integrates_a_log_singularity() {
        // ∫₀¹ ln(x) dx = -1
        let got = xc_integrate(&|x: f64| if x > 0.0 { x.ln() } else { 0.0 }, 0.0, 1.0);
        assert!((got + 1.0).abs() < 1e-9, "got {got}");
    }

    #[test]
    fn zero_width_interval_is_zero() {
        let got = xc_integrate(&|x: f64| x.exp(), 1.5, 1.5);
        assert_eq!(got, 0.0);
    }
}
