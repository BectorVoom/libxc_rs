//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 884/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk884(t10046: f64, t218: f64, t225: f64, t2592: f64, t2627: f64, t852: f64, t2633: f64, t235: f64, t860: f64, t9958: f64, t2679: f64, t2732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10047 = t218 * t10046;
    let t10049 = t2592 * t225;
    let t10054 = t2627 * t852;
    let t10055 = t10054 * t2633;
    let t10058 = t235 * t10046;
    let t10069 = t860 * t9958;
    let t10073 = t2732 * t2679;
    (t10047, t10049, t10055, t10058, t10069, t10073)
}
