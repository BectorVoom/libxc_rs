//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta631(t87291: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t23133: f64, t4257: f64, t1496: f64, t81942: f64, t7497: f64, t81933: f64, t25098: f64, t81835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t87292, t87293, t87295, t87301, t87304, t87306, t87308) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2043(t87291, t23062, t25106, t13176, t6613, t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835);
    (t87292, t87293, t87295, t87301, t87304, t87306, t87308)
}
