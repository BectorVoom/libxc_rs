//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 939/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk939(t10535: f64, t5089: f64, t11: f64, t10524: f64, t1691: f64, t10539: f64, t2704: f64, t10353: f64, t625: f64, t10357: f64, t10555: f64, t10550: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10560 = t5089 * t10535;
    let t10561 = t11 * t10560;
    let t10563 = t1691 * t10524;
    let t10564 = t11 * t10563;
    let t10566 = t1691 * t10539;
    let t10567 = t2704 * t10566;
    let t10569 = t625 * t10353;
    let t10570 = t11 * t10569;
    let t10572 = t625 * t10357;
    let t10573 = t2704 * t10572;
    let t10575 = t1691 * t10555;
    let t10576 = t11 * t10575;
    let t10578 = t625 * t10550;
    (t10561, t10564, t10567, t10570, t10573, t10576, t10578)
}
