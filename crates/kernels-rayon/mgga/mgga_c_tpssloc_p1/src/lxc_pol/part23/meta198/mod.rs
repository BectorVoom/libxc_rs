//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk836;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk837;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk838;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta198(t1239: f64, t68: f64, t2393: f64, t374: f64, t486: f64, t485: f64, t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t10469: f64, t466: f64, t10471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11604, t11606, t11647, t11649, t11668, t11677, t11678) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk836(t1239, t68, t2393, t374, t486, t485, t3585, t820, t10401, t3575, t3610);
        let t11692 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk837(t11677, t3624);
        let t11697 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk838(t3521, t820);
        let (t11712, t11713) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk839(t10469, t466, t10471);
    (t11604, t11606, t11647, t11649, t11668, t11677, t11678, t11692, t11697, t11712, t11713)
}
