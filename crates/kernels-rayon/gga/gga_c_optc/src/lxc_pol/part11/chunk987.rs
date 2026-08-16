//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 987/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk987(t18058: f64, t9118: f64, t9124: f64, t5311: f64, t8974: f64, t4458: f64, t9104: f64, t18012: f64, t4289: f64, t18023: f64, t3146: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18059 = t18058 * t9118;
    let t18062 = t18058 * t9124;
    let t18065 = t8974 * t5311;
    let t18066 = t4458 * t18065;
    let t18069 = t18058 * t9104;
    let t18072 = t4289 * t18012;
    let t18075 = t3146 * t18023;
    let t18076 = t894 * t18075;
    (t18059, t18062, t18065, t18066, t18069, t18072, t18075, t18076)
}
