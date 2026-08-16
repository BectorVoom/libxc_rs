//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2396;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta620(t246: f64, t40167: f64, t12126: f64, t588: f64, t39037: f64, t522: f64, t2221: f64, t3826: f64, t3824: f64, t12132: f64, t592: f64, t3696: f64, t2223: f64, t1336: f64, t1339: f64, t2691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40168, t40221, t40224, t40225, t40227, t40230, t40231) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2396(t246, t40167, t12126, t588, t39037, t522, t2221, t3826, t3824, t12132, t592, t3696);
        let (t40233, t40281) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2397(t2223, t3696, t1336, t1339, t2691);
    (t40168, t40221, t40224, t40225, t40227, t40230, t40231, t40233, t40281)
}
