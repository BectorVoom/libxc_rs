//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta32 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk233;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk234;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk235;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk236;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk237;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk238;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta32(t608: f64, t65: f64, t34: f64, t36: f64, rho0: f64, sigma0: f64, t43: f64, t607: f64, t55: f64, t583: f64, t61: f64, t59: f64, t39: f64, t44: f64, t51: f64, t33: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t609 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk233(t608, t65);
        let (t612, t614, t615) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk234(t34, t36, rho0, sigma0);
        let (t618, t621, t625) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk235(t43, t607, t55, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk236(t59, t625);
        let (t627, t628) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk237(t626, t39, t44, t51, t615, t618, t621);
        let t629 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk238(t33, t628);
        let t632 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk239(t40);
    (t609, t612, t614, t615, t618, t621, t625, t626, t627, t628, t629, t632)
}
