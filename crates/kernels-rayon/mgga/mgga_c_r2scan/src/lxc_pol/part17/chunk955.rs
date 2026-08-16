//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 955/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk955(t10964: f64, t10969: f64, t10973: f64, t10982: f64, t10990: f64, t10995: f64, t10741: f64, t10758: f64, t10818: f64, t10834: f64, t10842: f64, t10853: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11374 = 0.30487649791575028312e-3_f64 * t10964;
    let t11375 = 0.68400385060046895e-6_f64 * t10969;
    let t11377 = 0.60975299583150056624e-3_f64 * t10973;
    let t11378 = 0.86737941314158990616e-4_f64 * t10982;
    let t11379 = 0.29810146462873361016e-2_f64 * t10990;
    let t11380 = 0.60975299583150056624e-3_f64 * t10995;
    let t11393 = 0.31147743054556651237e-1_f64 * t10741;
    let t11399 = 0.11902492299418487743e0_f64 * t10758;
    let t11417 = 0.58544643236296698113e-1_f64 * t10818;
    let t11422 = 0.84755945902752848174e0_f64 * t10834;
    let t11425 = 0.32927245914677557993e-1_f64 * t10842;
    let t11428 = 0.16262400898971305031e-3_f64 * t10853;
    (t11374, t11375, t11377, t11378, t11379, t11380, t11393, t11399, t11417, t11422, t11425, t11428)
}
