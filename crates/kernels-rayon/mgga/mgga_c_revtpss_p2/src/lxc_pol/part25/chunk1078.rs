//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1078/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1078(t1214: f64, t2258: f64, t5296: f64, t1042: f64, t3617: f64, t3363: f64, t3172: f64, t3590: f64, t1247: f64, t11231: f64, t5302: f64, t3612: f64) -> (f64, f64, f64, f64, f64) {
    let t12931 = t2258 * t1214;
    let t12932 = t5296 * t12931;
    let t12933 = t1042 * t12932;
    let t12936 = t3617 * t1214;
    let t12937 = t12936 * t3363;
    let t12938 = t1042 * t12937;
    let t12941 = t3172 * t3590;
    let t12942 = t1247 * t12941;
    let t12944 = t5302 * t11231;
    let t12945 = t1042 * t12944;
    let t12948 = t3172 * t3612;
    (t12933, t12938, t12942, t12945, t12948)
}
