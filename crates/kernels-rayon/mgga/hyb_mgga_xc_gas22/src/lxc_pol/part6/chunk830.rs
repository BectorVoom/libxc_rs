//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 830/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk830(t1804: f64, t1809: f64, t6214: f64, t1815: f64, t765: f64, t136: f64, t2153: f64, t550: f64, t168: f64, t693: f64, t140: f64, t35: f64, t6007: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6216 = t1804 * t6214 * t1809;
    let t6226 = t1815 * t765;
    let t6227 = t136 * t6226;
    let t6229 = t550 * t2153;
    let t6230 = t136 * t6229;
    let t6270 = 1.0_f64 / t168 / t693;
    let t6278 = 14.0_f64 / 243.0_f64 * t35 * t6007 * t140;
    (t6216, t6226, t6227, t6229, t6230, t6270, t6278)
}
