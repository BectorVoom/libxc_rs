//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1450/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1450(t4757: f64, t5004: f64, t3291: f64, t6244: f64, t1082: f64, t19399: f64, t4866: f64, t4982: f64, t4893: f64, t1647: f64, t4980: f64, t1071: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19509 = t5004 * t4757;
    let t19512 = t3291 * t6244;
    let t19515 = t1082 * t19399;
    let t19520 = t4982 * t4866;
    let t19521 = t4893 * t19520;
    let t19526 = t1647 * t4980;
    let t19533 = t1071 * t6305;
    (t19509, t19512, t19515, t19521, t19526, t19533)
}
