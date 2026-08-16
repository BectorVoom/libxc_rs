//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1793/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1793(t33: f64, t5962: f64, t6079: f64, t1583: f64, t1711: f64, t6075: f64, t25826: f64, t5891: f64, t5915: f64, t6998: f64, t6846: f64, t7264: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29953 = t33 * t5962;
    let t29964 = t33 * t6079;
    let t29967 = t1711 * t1583;
    let t29970 = t33 * t6075;
    let t29999 = t25826 * t5891;
    let t30001 = t6998 * t5915;
    let t30035 = t7264 * t6846;
    (t29953, t29964, t29967, t29970, t29999, t30001, t30035)
}
