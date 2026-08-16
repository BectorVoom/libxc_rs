//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1047/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1047(t198: f64, t20157: f64, t565: f64, t595: f64, t1560: f64, t4360: f64, t4390: f64, t4250: f64, t874: f64, t20073: f64, t2366: f64, t10523: f64, t1422: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20158 = t565 * t198 * t20157;
    let t20168 = t565 * t595 * t20157;
    let t20172 = t565 * t1560 * t20157;
    let t20229 = t4360 * t4390;
    let t20237 = t4250 * t874;
    let t20358 = t2366 * t20073;
    let t20367 = t544 * t10523 * t1422;
    (t20158, t20168, t20172, t20229, t20237, t20358, t20367)
}
