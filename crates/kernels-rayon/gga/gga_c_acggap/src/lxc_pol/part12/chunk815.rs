//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 815/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk815(t157: f64, t406: f64, t556: f64, t309: f64, t525: f64, t1603: f64, t615: f64, t301: f64, t560: f64, t105: f64, t566: f64, t95: f64) -> (f64, f64, f64, f64, f64) {
    let t9025 = t556 * t406 * t157;
    let t9029 = t525 * t309;
    let t9058 = t615 * t1603;
    let t9089 = t560 * t301;
    let t9096 = t566 * t95 * t105;
    (t9025, t9029, t9058, t9089, t9096)
}
