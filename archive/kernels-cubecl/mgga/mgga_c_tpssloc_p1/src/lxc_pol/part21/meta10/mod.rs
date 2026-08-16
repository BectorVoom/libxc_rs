//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta10 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk74;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk75;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk76;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk77;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk78;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk79;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk80;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta10<F: Float>(t123: F, t126: F, t129: F, t136: F, t144: F, t159: F, t157: F, t153: F, t152: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t164, t167, t168) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk74::<F>(t123, t126, t129, t136);
        let t172 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk75::<F>(t123);
        let (t177, t180, t181) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk76::<F>(t123, t126, t129, t136);
        let t182 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk77::<F>(t172, t181);
        let t184 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk78::<F>(t144, t159, t168, t182);
        let t185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk79::<F>(t157, t184);
        let (t186, t187) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk80::<F>(t153, t185, t152, t157);
    (t164, t167, t168, t172, t177, t180, t181, t182, t184, t185, t186, t187)
}
