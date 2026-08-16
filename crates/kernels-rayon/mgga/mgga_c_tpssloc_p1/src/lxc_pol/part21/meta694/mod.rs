//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2517;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2518;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2519;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2520;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta694(t2394: f64, t4348: f64, t13612: f64, t690: f64, t13580: f64, t118: f64, t122: f64, t154: f64, t10277: f64, t1043: f64, t10216: f64, t3061: f64, t2770: f64, t376: f64, t1540: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t47730 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2517(t2394, t4348);
        let t47732 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2518(t13612, t690);
        let t47738 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2519(t13580, t690);
        let t47774 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2520(t118, t122, t154);
        let (t47775, t47779, t47783, t47787) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2521(t10277, t1043, t10216, t3061, t2770, t376, t1540, t9698);
    (t47730, t47732, t47738, t47774, t47775, t47779, t47783, t47787)
}
