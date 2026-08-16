//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 732/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk732(t406: f64, t8749: f64, t8697: f64, t2972: f64, t398: f64, t393: f64, t8639: f64, t8642: f64, t1065: f64, t2975: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8765 = t406 * t8749;
    let t8772 = t406 * t8697;
    let t8785 = 1.0_f64 / t2972 / t398;
    let t8786 = t393 * t8785;
    let t8831 = 0.16068111111111111111e1_f64 * t8639;
    let t8832 = 0.46308888888888888888e0_f64 * t8642;
    let t8847 = 1.0_f64 / t2972 / t1065;
    let t8848 = t393 * t8847;
    let t8850 = 1.0_f64 / t2975 / t401;
    let t8857 = 0.28842592592592592592e-1_f64 * t8639;
    let t8871 = 0.53272592592592592592e-1_f64 * t8639;
    let t8885 = 0.55403703703703703703e-1_f64 * t8639;
    (t8765, t8772, t8785, t8786, t8831, t8832, t8847, t8848, t8850, t8857, t8871, t8885)
}
