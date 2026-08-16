//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 436/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk436(t1121: f64, t3417: f64, t397: f64, t1111: f64, t1119: f64, t1125: f64, t119: f64, t268: f64, t275: f64, t3042: f64, t3392: f64, t3399: f64, t3406: f64, t3413: f64, t918: f64) -> (f64, f64) {
    let t3419 = t397 * t1121 * t3417;
    let t3422 = 0.5397236614853195164e-1_f64 * t3392 * t119 * t275 - 0.25187104202648244098e0_f64 * t1111 * t918 * t275 - 0.10794473229706390328e0_f64 * t3399 * t1125 + 0.41978507004413740163e0_f64 * t268 * t3042 * t275 + 0.25187104202648244098e0_f64 * t3406 * t1125 + 0.10794473229706390328e0_f64 * t1119 * t3413 - 0.5397236614853195164e-1_f64 * t1119 * t3419;
    (t3419, t3422)
}
