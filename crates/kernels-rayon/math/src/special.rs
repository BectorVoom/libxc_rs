//! Special mathematical functions for XC functional evaluation.
//!
//! Translated from libxc `special_functions.c` and `faddeeva.c`.

#![allow(
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]


// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

// ============================================================================
// Chebyshev evaluation helper
// ============================================================================

/// Evaluate a Chebyshev series at point x using Clenshaw recurrence.
/// `x` must be in [-1, 1]. `coeffs` are the Chebyshev coefficients.
/// `n` is the number of coefficients to use.
///
/// This is a manual unrolling since CubeCL doesn't support dynamic loops
/// over arrays. We support up to 38 coefficients (needed for dilogarithm).
fn cheb_eval_38(x: f64, c0: f64, c1: f64, c2: f64, c3: f64, c4: f64,
    c5: f64, c6: f64, c7: f64, c8: f64, c9: f64,
    c10: f64, c11: f64, c12: f64, c13: f64, c14: f64,
    c15: f64, c16: f64, c17: f64, c18: f64, c19: f64,
    c20: f64, c21: f64, c22: f64, c23: f64, c24: f64,
    c25: f64, c26: f64, c27: f64, c28: f64, c29: f64,
    c30: f64, c31: f64, c32: f64, c33: f64, c34: f64,
    c35: f64, c36: f64, c37: f64,
) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;

    // Clenshaw recurrence from last coefficient to first
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c37;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c36;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c35;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c34;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c33;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c32;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c31;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c30;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c29;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c28;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c27;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c26;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c25;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c24;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c23;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c22;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c21;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c20;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c19;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c18;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c17;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c16;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c15;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c14;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c13;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c12;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c11;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c10;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c9;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c8;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c7;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c6;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c5;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c4;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c3;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c2;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c1;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c0;

    0.5_f64 * (b0 - b2)
}

// ============================================================================
// Dilogarithm (Spence's function)
// ============================================================================

// Chebyshev coefficients for the Spence function (from SLATEC / libxc)
// 38 coefficients

/// Compute the dilogarithm Li_2(x) = -∫₀ˣ ln(1-t)/t dt.
///
/// Translated from libxc `special_functions.c` `xc_dilogarithm`.
/// Uses Chebyshev approximation (SLATEC method by W. Fullerton).
///
/// Note: precision-sensitive coefficients (pi26, big, s0..s37) are
/// declared via (<f64-literal> as f64) to preserve full f64 precision
/// in f64 mode (Rule 3 spirit — let-bindings are f64-precision constants
/// the same as named const items would be).
pub fn xc_dilogarithm(x: f64) -> f64 {
    let pi26: f64 = 1.644934066848226436472415166646025189219_f64;

    // Spence Chebyshev coefficients (38 terms)
    let s0:  f64 = 0.1527365598892405872946684910028e0_f64;
    let s1:  f64 = 0.8169658058051014403501838185271e-1_f64;
    let s2:  f64 = 0.5814157140778730872977350641182e-2_f64;
    let s3:  f64 = 0.5371619814541527542247889005319e-3_f64;
    let s4:  f64 = 0.5724704675185826233210603054782e-4_f64;
    let s5:  f64 = 0.6674546121649336343607835438589e-5_f64;
    let s6:  f64 = 0.8276467339715676981584391689011e-6_f64;
    let s7:  f64 = 0.1073315673030678951270005873354e-6_f64;
    let s8:  f64 = 0.1440077294303239402334590331513e-7_f64;
    let s9:  f64 = 0.1984442029965906367898877139608e-8_f64;
    let s10: f64 = 0.2794005822163638720201994821615e-9_f64;
    let s11: f64 = 0.4003991310883311823072580445908e-10_f64;
    let s12: f64 = 0.5823462892044638471368135835757e-11_f64;
    let s13: f64 = 0.8576708692638689278097914771224e-12_f64;
    let s14: f64 = 0.1276862586280193045989483033433e-12_f64;
    let s15: f64 = 0.1918826209042517081162380416062e-13_f64;
    let s16: f64 = 0.2907319206977138177795799719673e-14_f64;
    let s17: f64 = 0.4437112685276780462557473641745e-15_f64;
    let s18: f64 = 0.6815727787414599527867359135607e-16_f64;
    let s19: f64 = 0.1053017386015574429547019416644e-16_f64;
    let s20: f64 = 0.1635389806752377100051821734570e-17_f64;
    let s21: f64 = 0.2551852874940463932310901642581e-18_f64;
    let s22: f64 = 0.3999020621999360112770470379519e-19_f64;
    let s23: f64 = 0.6291501645216811876514149171199e-20_f64;
    let s24: f64 = 0.9933827435675677643803887752533e-21_f64;
    let s25: f64 = 0.1573679570749964816721763805866e-21_f64;
    let s26: f64 = 0.2500595316849476129369270954666e-22_f64;
    let s27: f64 = 0.3984740918383811139210663253333e-23_f64;
    let s28: f64 = 0.6366473210082843892691326293333e-24_f64;
    let s29: f64 = 0.1019674287239678367077061973333e-24_f64;
    let s30: f64 = 0.1636881058913518841111074133333e-25_f64;
    let s31: f64 = 0.2633310439417650117345279999999e-26_f64;
    let s32: f64 = 0.4244811560123976817224362666666e-27_f64;
    let s33: f64 = 0.6855411983680052916824746666666e-28_f64;
    let s34: f64 = 0.1109122433438056434018986666666e-28_f64;
    let s35: f64 = 0.1797431304999891457365333333333e-29_f64;
    let s36: f64 = 0.2917505845976095173290666666666e-30_f64;
    let s37: f64 = 0.4742646808928671061333333333333e-31_f64;

    // FLT_RADIX/DBL_EPSILON ≈ 2/2.2e-16 ≈ 9.0e15
    let big: f64 = 9.007199254740992e15_f64;

    let mut dspenc: f64 = 0.0_f64;

    if x > 2.0_f64 {
        let aux = rmath::ln(x);
        dspenc = 2.0_f64 * pi26 - 0.5_f64 * aux * aux;
        if x < big {
            dspenc = dspenc - (1.0_f64 + cheb_eval_38(4.0_f64 / x - 1.0_f64,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / x;
        }
    } else if x > 1.0_f64 {
        let aux = x - 1.0_f64;
        dspenc = pi26 - 0.5_f64 * rmath::ln(x) * rmath::ln(aux * aux / x)
            + aux * (1.0_f64 + cheb_eval_38(4.0_f64 * aux / x - 1.0_f64,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / x;
    } else if x > 0.5_f64 {
        // x != 1.0 case (x > 0.5 && x <= 1.0, and we already handled x > 1.0)
        dspenc = pi26 - rmath::ln(x) * rmath::ln(1.0_f64 - x)
            - (1.0_f64 - x) * (1.0_f64 + cheb_eval_38(4.0_f64 * (1.0_f64 - x) - 1.0_f64,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37));
    } else if x >= 0.0_f64 {
        dspenc = x * (1.0_f64 + cheb_eval_38(4.0_f64 * x - 1.0_f64,
            s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
            s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
            s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
            s30, s31, s32, s33, s34, s35, s36, s37));
    } else if x > -1.0_f64 {
        let aux = rmath::ln(1.0_f64 - x);
        dspenc = -0.5_f64 * aux * aux
            - x * (1.0_f64 + cheb_eval_38(4.0_f64 * x / (x - 1.0_f64) - 1.0_f64,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / (x - 1.0_f64);
    } else {
        let aux = rmath::ln(1.0_f64 - x);
        dspenc = -pi26 - 0.5_f64 * aux * (2.0_f64 * rmath::ln(-x) - aux);
        if x > -big {
            dspenc = dspenc + (1.0_f64 + cheb_eval_38(4.0_f64 / (1.0_f64 - x) - 1.0_f64,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / (1.0_f64 - x);
        }
    }

    dspenc
}

// ============================================================================
// erfcx — scaled complementary error function: erfcx(x) = exp(x²) * erfc(x)
// ============================================================================

/// `erfcx(x) = exp(x^2) * erfc(x)`.
///
/// Transcribed from libxc `faddeeva.c::xc_erfcx`, including its two
/// continued-fraction shortcuts for large positive `x` and the reflection for
/// negative `x`. The core range defers to [`erfcx_table::erfcx_y100`], which is
/// libxc's own 100-interval Chebyshev look-up.
///
/// Verified bit-for-bit against libxc's C by
/// `verify/tests/screening_helpers.rs`. That test exists because this function
/// is reached only on the screened-exchange path -- `gga_x_wpbeh` at its
/// default `_omega = 0` never calls it -- so it went unverified for as long as
/// nothing evaluated a screened hybrid.
pub fn xc_erfcx(x: f64) -> f64 {
    const ISPI: f64 = 0.56418958354775628694807945156_f64; // 1 / sqrt(pi)

    if x >= 0.0_f64 {
        if x > 50.0_f64 {
            // Continued-fraction expansion is faster out here.
            if x > 5.0e7_f64 {
                // 1-term expansion; important to avoid overflow in x*x.
                return ISPI / x;
            }
            // 5-term expansion, simplified from
            //   ispi / (x + 0.5/(x + 1/(x + 1.5/(x + 2/x))))
            return ISPI * ((x * x) * (x * x + 4.5_f64) + 2.0_f64)
                / (x * ((x * x) * (x * x + 5.0_f64) + 3.75_f64));
        }
        crate::erfcx_table::erfcx_y100(400.0_f64 / (4.0_f64 + x))
    } else if x < -26.7_f64 {
        f64::INFINITY
    } else if x < -6.1_f64 {
        2.0_f64 * rmath::exp(x * x)
    } else {
        2.0_f64 * rmath::exp(x * x) - crate::erfcx_table::erfcx_y100(400.0_f64 / (4.0_f64 - x))
    }
}
