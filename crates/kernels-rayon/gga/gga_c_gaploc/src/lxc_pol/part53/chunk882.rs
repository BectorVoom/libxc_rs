//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 882/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk882(t2268: f64, t2304: f64, t34273: f64, t39849: f64, t12803: f64, t29874: f64, t31586: f64, t4261: f64, t9074: f64, t1063: f64, t2854: f64, t29969: f64, t6320: f64) -> (f64, f64, f64, f64, f64) {
    let t42844 = 0.39837009289946609438e0_f64 * t2268 * t2304 * t34273;
    let t42845 = 0.142275033178380748e-1_f64 * t39849;
    let t42846 = t29874 * t12803;
    let t42847 = 0.47425011059460249332e-2_f64 * t42846;
    let t42849 = t9074 * t4261 * t31586;
    let t42850 = 0.47425011059460249332e-2_f64 * t42849;
    let t42857 = 0.17073003981405689759e0_f64 * t1063 * t6320 * t2854 * t29969;
    (t42844, t42845, t42847, t42850, t42857)
}
