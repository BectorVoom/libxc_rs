//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta15 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk118;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk119;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk120;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk121;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk122;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk123;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk124;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta15(t235: f64, t252: f64, t226: f64, t254: f64, t253: f64, t144: f64, t186: f64, t189: f64, t193: f64, t202: f64, t118: f64, t120: f64, t121: f64, t154: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t255 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk118(t235, t252);
        let (t257, t259) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk119(t226, t255, t254);
        let (t261, t262) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk120(t253, t259);
        let t265 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk121(t144, t186, t189, t193, t202, t262);
        let t268 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk122(t118, t120);
        let (t269, t270, t271) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk123(t121, t154, t40);
        let t273 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk124(t268, t269, t271);
        let t275 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk125(t273);
    (t255, t257, t259, t261, t262, t265, t268, t269, t270, t271, t273, t275)
}
