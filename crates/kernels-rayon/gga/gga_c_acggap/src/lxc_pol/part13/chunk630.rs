//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 630/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk630(t174: f64, t4875: f64, t386: f64, t428: f64, t3228: f64, t532: f64, t1008: f64, t1569: f64, t3266: f64, t422: f64, t530: f64, t3670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4876 = t174 * t4875;
    let t4878 = t386 * t428 * t4876;
    let t4881 = t3228 * t532;
    let t4884 = 0.17149607247227894789e-2_f64 * t1008 * t1569;
    let t4886 = t422 * t3266 * t530;
    let t4889 = t3670 * t532;
    (t4876, t4878, t4881, t4884, t4886, t4889)
}
