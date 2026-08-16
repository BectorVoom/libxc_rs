//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta446(t22724: f64, t6973: f64, t6982: f64, t794: f64, t6897: f64, t6883: f64, t6983: f64, t6914: f64, t6979: f64, t6546: f64, t6887: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t22726, t22727, t22728, t22730, t22745, t22751) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1707(t22724, t6973, t6982, t794, t6897, t6883, t6983, t6914, t6979, t6546, t6887);
    (t22726, t22727, t22728, t22730, t22745, t22751)
}
