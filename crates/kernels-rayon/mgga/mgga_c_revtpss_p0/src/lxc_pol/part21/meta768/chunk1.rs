//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2722/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722(t198: f64, t775: f64, t10565: f64, t1469: f64, t706: f64, t1531: f64, t36: f64, t10440: f64, t14362: f64, t9863: f64, t9866: f64, t40143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50080 = t198 * t775;
    let t50084 = t706 * t10565 * t1469;
    let t50085 = 4.0_f64 * t50084;
    let t50089 = t36 * t1531;
    let t50091 = 24.0_f64 * t50089 * t10440;
    let t50092 = t14362 * t9863;
    let t50093 = 0.16265371950452609763e-1_f64 * t50092;
    let t50094 = t14362 * t9866;
    let t50095 = 0.48159733137676571078e0_f64 * t50094;
    let t50096 = 36.0_f64 * t40143;
    (t50080, t50085, t50091, t50093, t50095, t50096)
}
