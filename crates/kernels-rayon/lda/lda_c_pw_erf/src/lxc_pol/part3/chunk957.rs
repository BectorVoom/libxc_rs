//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 957/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk957(t169: f64, t2877: f64, t632: f64, t10770: f64, t242: f64, t2881: f64, t1098: f64, t1143: f64, t1: f64, t10752: f64, t10780: f64, t10784: f64, t1132: f64, t1135: f64, t1139: f64, t1140: f64, t119: f64, t159: f64, t161: f64, t20: f64, t2782: f64, t2824: f64, t2869: f64, t2911: f64, t2917: f64, t2920: f64, t2923: f64, t3: f64, t343: f64, t39: f64, t477: f64, t628: f64, t629: f64) -> (f64, f64, f64, f64, f64) {
    let t10913 = 2.0752137690161367_f64 * t169 * t2877 * t632;
    let t10915 = t169 * t10770 * t242;
    let t10918 = t169 * t2881 * t632;
    let t10922 = 0.8489510873247833_f64 * t169 * t1098 * t1143;
    let t10953 = t10752 / 2.0_f64 + 0.1254_f64 * t10780 * t3 * t629 - 0.2508_f64 * t2911 * t1135 + 0.04717548_f64 * t10784 * t20 * t1140 + 0.39013333333333333_f64 * t1132 * t2920 - 0.12580128_f64 * t2917 * t2923 + 0.007532237109403992_f64 * t477 * t39 * t161 - 0.32511111111111113_f64 * t628 * t2869 * t161 + 0.1397792_f64 * t1139 * t2782 * t161 - 0.015064474218807983_f64 * t159 * t343 * t161 + 0.00011806781668990758_f64 * t159 * t2824 * t1 * t119 * t161;
    (t10913, t10915, t10918, t10922, t10953)
}
