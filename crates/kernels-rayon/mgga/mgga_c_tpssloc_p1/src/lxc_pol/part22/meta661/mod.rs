//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2205;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta661(t12984: f64, t12998: f64, t4119: f64, t686: f64, t5555: f64, t9541: f64, t41008: f64, t5550: f64, t16783: f64, t41196: f64, t118: f64, t16662: f64, t2576: f64, t794: f64, t16787: f64, t2563: f64, t16791: f64, t9546: f64, t2586: f64, t41146: f64, t59162: f64, t59135: f64, t9523: f64, t5624: f64, t9993: f64, t5628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59173, t59195, t59204, t59206, t59214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2205(t12984, t12998, t4119, t686, t5555, t9541, t41008, t5550, t16783, t41196, t118, t16662, t2576, t794);
        let (t59216, t59218, t59221, t59224, t59251, t59255) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2206(t16787, t2563, t16791, t9546, t2586, t41146, t59162, t59135, t9523, t5624, t9993, t5628);
    (t59173, t59195, t59204, t59206, t59214, t59216, t59218, t59221, t59224, t59251, t59255)
}
