//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk239;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk240;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk241;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk242;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk243;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk244;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk245;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta33(t626: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t621: f64, t33: f64, t40: f64, t73: f64, t52: f64, t76: f64, t607: f64, t72: f64, t609: f64, t66: f64, t80: f64, t5: f64, t601: f64, t605: f64, t86: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t627, t628, t629, t632) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk239(t626, t39, t44, t51, t615, t618, t621, t33, t40);
        let t634 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk240(t632, t73);
        let t636 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk241(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk242(t636, t76);
        let t641 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk243(t607, t634, t638);
        let (t642, t645) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk244(t641, t72, t609, t629, t66, t80);
        let t649 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk245(t5, t601, t605, t645, t86);
        let t650 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk246(t112, t649);
    (t627, t628, t629, t632, t634, t636, t638, t641, t642, t645, t649, t650)
}
