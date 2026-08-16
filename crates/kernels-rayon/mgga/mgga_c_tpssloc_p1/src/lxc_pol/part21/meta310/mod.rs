//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta310 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1660;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1661;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1662;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1663;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1664;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta310(t2393: f64, t374: f64, t486: f64, t485: f64, t248: f64, t3516: f64, t3570: f64, t3515: f64, t3576: f64, t3604: f64, t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11647, t11649, t11651, t11652, t11665) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1660(t2393, t374, t486, t485, t248, t3516, t3570, t3515, t3576, t3604);
        let t11668 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1661(t3585, t820);
        let t11677 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1662(t10401, t3575);
        let t11678 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1663(t11677, t3610);
        let t11692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1664(t11677, t3624);
        let t11697 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1665(t3521, t820);
    (t11647, t11649, t11651, t11652, t11665, t11668, t11677, t11678, t11692, t11697)
}
