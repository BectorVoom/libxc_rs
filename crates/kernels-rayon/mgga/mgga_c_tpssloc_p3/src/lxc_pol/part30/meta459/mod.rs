//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1734;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1735;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1736;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta459(t234: f64, t852: f64, t117: f64, t229: f64, t67: f64, t6559: f64, t22893: f64, t6639: f64, t6546: f64, t6551: f64, t6640: f64, t22641: f64, t2587: f64, t22690: f64, t6638: f64, t206: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23153, t23163, t23164) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1734(t234, t852, t117, t229, t67, t6559);
        let (t23165, t23166, t23168) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1735(t22893, t6639, t23164, t6546, t6551);
        let (t23169, t23171) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1736(t23168, t6640, t22641, t2587);
        let (t23172, t23174, t23185) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1737(t22690, t6638, t23171, t206, t268, t6559);
    (t23153, t23163, t23164, t23165, t23166, t23168, t23169, t23171, t23172, t23174, t23185)
}
