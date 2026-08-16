//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta3 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk22;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk23;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk24;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk25;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk26;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk27;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk28;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk29;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk30;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta3(t31: f64, sigma0: f64, sigma1: f64, sigma2: f64, t3: f64, t10: f64, t39: f64, t44: f64, t51: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t52 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk22(t31);
        let (t53, t54, t55) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk23(t52);
        let (t56, t59) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk24(t53, t55, sigma0, sigma1, sigma2);
        let t60 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk25(t3);
        let t61 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk26(t60);
        let t63 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk27(t10, t61);
        let t64 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk28(t59, t63);
        let t65 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk29(t39, t44, t51, t56, t64);
        let t66 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk30(t33, t65);
    (t52, t53, t54, t55, t56, t59, t60, t61, t63, t64, t65, t66)
}
