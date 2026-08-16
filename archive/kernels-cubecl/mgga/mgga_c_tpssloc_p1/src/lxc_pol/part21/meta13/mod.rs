//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta13 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk98;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk99;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk100;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk101;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk102;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk103;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk104;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk105;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta13<F: Float>(t191: F, t218: F, t144: F, t186: F, t189: F, t202: F, t68: F, t59: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t225 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk98::<F>(t191);
        let t226 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk99::<F>(t218, t225);
        let t228 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk100::<F>(t144, t186, t189, t225);
        let (t229, t230) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk101::<F>(t202, t68);
        let t232 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk102::<F>(t228, t230);
        let (t233, t234, t235) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk103::<F>(t232, t68);
        let t236 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk104::<F>(t59);
        let t237 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk105::<F>(t235, t236);
        let t238 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk106::<F>(t226, t237);
    (t225, t226, t228, t229, t230, t232, t233, t234, t235, t236, t237, t238)
}
