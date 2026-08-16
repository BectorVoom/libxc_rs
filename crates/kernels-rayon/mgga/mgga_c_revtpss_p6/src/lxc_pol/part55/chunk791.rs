//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 791/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk791(t257: f64, t827: f64, t828: f64, t1955: f64, t239: f64, t8464: f64, t1954: f64, t209: f64, t2452: f64) -> (f64, f64, f64, f64) {
    let t8468 = t827 * t828 * t257;
    let t8469 = t1955 * t8464 * t239 * t8468;
    let t8476 = t1954 * t209;
    let t8477 = t8476 * t2452;
    (t8468, t8469, t8476, t8477)
}
