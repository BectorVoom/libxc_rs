//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2205;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta661<F: Float>(t12984: F, t12998: F, t4119: F, t686: F, t5555: F, t9541: F, t41008: F, t5550: F, t16783: F, t41196: F, t118: F, t16662: F, t2576: F, t794: F, t16787: F, t2563: F, t16791: F, t9546: F, t2586: F, t41146: F, t59162: F, t59135: F, t9523: F, t5624: F, t9993: F, t5628: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59173, t59195, t59204, t59206, t59214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2205::<F>(t12984, t12998, t4119, t686, t5555, t9541, t41008, t5550, t16783, t41196, t118, t16662, t2576, t794);
        let (t59216, t59218, t59221, t59224, t59251, t59255) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2206::<F>(t16787, t2563, t16791, t9546, t2586, t41146, t59162, t59135, t9523, t5624, t9993, t5628);
    (t59173, t59195, t59204, t59206, t59214, t59216, t59218, t59221, t59224, t59251, t59255)
}
