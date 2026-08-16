//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 301/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk301(t106: f64, t797: f64, t97: f64, t986: f64, t889: f64, t298: f64, t302: f64, t308: f64, t295: f64, t305: f64, t309: f64, t814: f64, rho1: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t989 = t97 * t106 * t986 * t797;
    let t990 = t889 / 2.0_f64;
    let t991 = t298 * t990;
    let t994 = rho1 * rho1;
    let t996 = 1.0_f64 / t302 / t994;
    let t997 = tau1 * t996;
    let t1000 = -t990;
    let t1001 = t308 * t1000;
    let t1004 = 5.0_f64 / 3.0_f64 * t295 * t991 - 5.0_f64 / 3.0_f64 * t997 * t309 + 5.0_f64 / 3.0_f64 * t305 * t1001 + t814;
    (t989, t990, t991, t997, t1000, t1001, t1004)
}
