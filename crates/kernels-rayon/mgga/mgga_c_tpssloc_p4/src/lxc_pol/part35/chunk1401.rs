//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1401/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1401(t20416: f64, t6888: f64, t6889: f64, t6890: f64, t20465: f64, t22833: f64, t20475: f64, t26309: f64, t20460: f64, t20454: f64, t26233: f64, t6422: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t107056 = t6888 * t6889 * t6890 * t20416;
    let t107063 = t22833 * t20465;
    let t107065 = t26309 * t20475;
    let t107067 = t22833 * t20460;
    let t107070 = t22833 * t20454;
    let t107074 = t26233 * t6422;
    (t107056, t107063, t107065, t107067, t107070, t107074)
}
