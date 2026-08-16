//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1886/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1886(t25222: f64, t4435: f64, t14868: f64, t2661: f64, t93082: f64, t14757: f64, t25234: f64, t14732: f64, t25245: f64, t14933: f64, t2482: f64, t25260: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    let t99073 = t25234 * t14757;
    let t99077 = t25245 * t14732;
    let t99085 = t2482 * t25260 * t814 * t14933;
    (t99066, t99069, t99073, t99077, t99085)
}
