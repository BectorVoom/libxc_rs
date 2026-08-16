//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 979/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk979(t2268: f64, t2304: f64, t34273: f64, t39849: f64, t12803: f64, t29874: f64, t31586: f64, t4261: f64, t9074: f64, t1358: f64, t42433: f64, t6507: f64) -> (f64, f64, f64, f64, f64) {
    let t42844 = 0.39837009289946609438e0_f64 * t2268 * t2304 * t34273;
    let t42845 = 0.142275033178380748e-1_f64 * t39849;
    let t42846 = t29874 * t12803;
    let t42847 = 0.47425011059460249332e-2_f64 * t42846;
    let t42849 = t9074 * t4261 * t31586;
    let t42850 = 0.47425011059460249332e-2_f64 * t42849;
    let t42852 = t1358 * t6507 * t42433;
    (t42844, t42845, t42847, t42850, t42852)
}
