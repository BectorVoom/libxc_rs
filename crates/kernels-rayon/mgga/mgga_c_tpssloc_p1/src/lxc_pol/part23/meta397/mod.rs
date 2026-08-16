//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1203;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta397(t16616: f64, t2528: f64, t212: f64, t5544: f64, t5527: f64, t5555: f64, t9541: f64, t41008: f64, t5550: f64, t16783: f64, t41196: f64, t16791: f64, t9546: f64, t2586: f64, t41146: f64, t9523: f64, t1516: f64, t47275: f64, t5628: f64, t9601: f64, t5619: f64, t9671: f64, t16673: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59028, t59135, t59162, t59195, t59204, t59206, t59218) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1203(t16616, t2528, t212, t5544, t5527, t5555, t9541, t41008, t5550, t16783, t41196, t16791, t9546);
        let (t59221, t59224, t59259, t59263, t59276, t59281) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1204(t2586, t41146, t59162, t59135, t9523, t1516, t47275, t5628, t9601, t5619, t9671, t16673, t2638);
    (t59028, t59195, t59204, t59206, t59218, t59221, t59224, t59259, t59263, t59276, t59281)
}
