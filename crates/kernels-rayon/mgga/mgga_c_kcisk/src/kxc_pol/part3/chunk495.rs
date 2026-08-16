//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 495/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk495(t3913: f64, t470: f64, t468: f64, t415: f64, t1406: f64, t1446: f64, t1327: f64, t408: f64, t1218: f64, t411: f64, t338: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3914 = sigma0 * t3913;
    let t3915 = t3914 * t470;
    let t3916 = t468 * t3915;
    let t3917 = t415 * t3916;
    let t3919 = t1406 * t1446;
    let t3920 = t415 * t3919;
    let t3922 = t1327 * t1327;
    let t3923 = t408 * t408;
    let t3924 = 1.0_f64 / t3923;
    let t3925 = t3922 * t3924;
    let t3929 = 1.0_f64 / t1218 / t411;
    let t3930 = t338 * t3929;
    (t3914, t3915, t3916, t3917, t3919, t3920, t3922, t3923, t3924, t3925, t3929, t3930)
}
