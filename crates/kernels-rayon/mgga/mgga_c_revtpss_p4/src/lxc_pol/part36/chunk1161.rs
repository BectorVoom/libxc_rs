//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1161/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1161(t29135: f64, t7642: f64, t2148: f64, t11239: f64, t1276: f64, t3596: f64, t2149: f64, t1243: f64, t460: f64, t8190: f64, t1209: f64, t1770: f64, t2142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29136 = t7642 * t29135;
    let t29141 = t2148 * t29135;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29207 = t460 * t8190;
    let t29220 = t1209 * t8190;
    let t29227 = t1770 * t2142;
    (t29136, t29141, t29193, t29194, t29199, t29200, t29207, t29220, t29227)
}
