//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1194/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1194(t18030: f64, t3244: f64, t9142: f64, t16011: f64, t4512: f64, t18019: f64, t11885: f64, t18012: f64, t1179: f64, t18075: f64, t2586: f64, t12802: f64, t16004: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54850 = t3244 * t9142 * t18030;
    let t54853 = t16011 * t4512;
    let t54904 = t3244 * t9142 * t18019;
    let t54911 = t3244 * t11885 * t18012;
    let t54926 = t1179 * t2586 * t18075;
    let t54941 = t12802 * t16004;
    (t54850, t54853, t54904, t54911, t54926, t54941)
}
