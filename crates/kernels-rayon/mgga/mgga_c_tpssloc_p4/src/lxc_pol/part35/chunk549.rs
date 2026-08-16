//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 549/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk549(t1573: f64, t942: f64, t1580: f64, t2932: f64, t300: f64, t2904: f64, t1592: f64, t2970: f64, t973: f64, t2978: f64, t60: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4449 = t1573 * t942;
    let t4475 = t1580 * t2932;
    let t4483 = t300 * t1573;
    let t4488 = t2904 * t1580;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    (t4449, t4475, t4483, t4488, t4507, t4509, t4510)
}
