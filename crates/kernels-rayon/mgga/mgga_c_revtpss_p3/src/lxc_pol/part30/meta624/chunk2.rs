//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2152/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2152(t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t27221: f64, t50789: f64, t50931: f64, t1549: f64, t92968: f64, t14697: f64, t25270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99041 = t807 * t1945 * t14741;
    let t99042 = 0.11433071498151929859e-3_f64 * t99041;
    let t99044 = t10886 * t7028 * t4416;
    let t99046 = t27221 * t50789;
    let t99048 = t27221 * t50931;
    let t99050 = t92968 * t1549;
    let t99052 = t25270 * t14697;
    (t99042, t99044, t99046, t99048, t99050, t99052)
}
