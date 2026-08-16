//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 325/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk325(t1011: f64, t1036: f64, t1017: f64, t1028: f64, t1033: f64, t1040: f64) -> (f64, f64, f64) {
    let t1056 = 0.516475e0_f64 * t1011;
    let t1059 = 0.104195e0_f64 * t1036;
    let t1061 = 0.3529725e1_f64 * t1028 - t1056 + 0.516475e0_f64 * t1017 + 0.6311625e0_f64 * t1033 - t1059 + 0.104195e0_f64 * t1040;
    (t1056, t1059, t1061)
}
