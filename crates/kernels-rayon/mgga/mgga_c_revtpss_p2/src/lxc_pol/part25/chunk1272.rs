//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1272/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1272(t11916: f64, t25509: f64, t25569: f64, t3111: f64, t11722: f64, t7132: f64, t11727: f64, t12002: f64, t1971: f64, t351: f64, t1052: f64, t3089: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93573 = t25509 * t11916;
    let t93579 = t25569 * t3111;
    let t93583 = t7132 * t11722;
    let t93585 = t7132 * t11727;
    let t93592 = t351 * t1971 * t12002;
    let t93595 = sigma0 * t1052;
    let t93596 = t93595 * t3089;
    (t93573, t93579, t93583, t93585, t93592, t93596)
}
