//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1211/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1211(t624: f64, t922: f64, t560: f64, t839: f64, t10409: f64, t1427: f64, t15026: f64, t2254: f64, t2355: f64, t24589: f64, t2541: f64, t29943: f64, t29948: f64, t29961: f64, t32246: f64, t32249: f64, t4818: f64, t4822: f64, t5439: f64, t567: f64, t625: f64, t7278: f64, t7297: f64, t8031: f64, t8372: f64) -> f64 {
    let t36621 = t922 * t624;
    let t36647 = t560 * t839;
    let t36654 = -6.0_f64 * t10409 * t5439 * t7297 + 12.0_f64 * t1427 * t29948 * t8372 - t15026 * t567 * t625 + 3.0_f64 * t2254 * t29943 * t567 + 6.0_f64 * t2254 * t36621 * t567 - t2355 * t567 * t8031 - 6.0_f64 * t24589 * t2541 * t7297 - 3.0_f64 * t2541 * t36647 * t7297 + 12.0_f64 * t4818 * t7278 * t8372 + 6.0_f64 * t4822 * t7278 * t8372 + 2.0_f64 * t29961 + 3.0_f64 * t32246 - 2.0_f64 * t32249;
    t36654
}
