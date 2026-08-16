//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk493;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk494;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk495;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta85(t1878: f64, t268: f64, t271: f64, t1043: f64, t154: f64, t632: f64, t2289: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2764, t2765, t2768) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk493(t1878, t268, t271, t1043, t154);
        let t2769 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk494(t632);
        let t2770 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk495(t2769);
        let t2775 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk496(t2289);
    (t2764, t2765, t2768, t2769, t2770, t2775)
}
