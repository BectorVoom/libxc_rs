//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1466/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1466(t19456: f64, t247: f64, t3116: f64, t3172: f64, t6311: f64, t3161: f64, t1043: f64, t6244: f64, t1045: f64, t3117: f64, t1668: f64, t4772: f64) -> (f64, f64, f64, f64, f64) {
    let t19819 = t247 * t3116 * t19456;
    let t19826 = t3172 * t6311;
    let t19827 = t3161 * t19826;
    let t19829 = t6244 * t1043;
    let t19830 = t19829 * t1045;
    let t19831 = t3117 * t19830;
    let t19836 = t4772 * t1668;
    (t19819, t19827, t19829, t19831, t19836)
}
