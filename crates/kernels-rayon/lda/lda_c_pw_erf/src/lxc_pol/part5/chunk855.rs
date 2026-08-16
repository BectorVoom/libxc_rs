//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 855/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk855(t120: f64, t7913: f64, t102: f64, t436: f64, t3296: f64, t7918: f64, t7159: f64, t7162: f64, t2610: f64, t763: f64, t127: f64, t1852: f64, t3313: f64, t3322: f64, t7143: f64, t7146: f64, t7149: f64, t7152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7924 = t120 * t7913;
    let t7926 = 2.923025_f64 * t102 * t7924;
    let t7927 = t436 * t7913;
    let t7930 = t3296 * t7918;
    let t7933 = t436 * t7918;
    let t7935 = 17.53815_f64 * t102 * t7933;
    let t7940 = 2.923025_f64 * t7159;
    let t7941 = 1.4615125_f64 * t7162;
    let t7947 = 17.53815_f64 * t102 * t763 * t2610;
    let t7948 = -t7926 - 1.46904_f64 * t127 * t7927 - 29.3808_f64 * t127 * t7930 - t7935 - 3.0_f64 / 2.0_f64 * t7143 + t7146 / 2.0_f64 - 8.81424_f64 * t7149 + 2.20356_f64 * t7152 - t7940 + t7941 + t3313 - t3322 + 17.62848_f64 * t127 * t1852 * t2610 + t7947;
    (t7924, t7926, t7927, t7930, t7933, t7935, t7940, t7941, t7947, t7948)
}
