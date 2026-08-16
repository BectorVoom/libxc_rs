//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1589;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1590;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1591;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1592;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1593;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta288(t10469: f64, t349: f64, t1011: f64, t1013: f64, t363: f64, t3034: f64, t6793: f64, t368: f64, t3131: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10470 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1589(t10469, t349);
        let t10471 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1590(t1011);
        let t10472 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1591(t10470, t10471);
        let (t10473, t10474, t10475, t10477) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1592(t1013, t363, t3034, t6793);
        let (t10478, t10479, t10480) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1593(t10477, t368, t10475, t10472);
        let t10482 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1594(t3131, t360);
    (t10470, t10471, t10472, t10473, t10474, t10475, t10477, t10478, t10479, t10480, t10482)
}
