//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1685/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1685(t2018: f64, t3951: f64, t807: f64, t1941: f64, t550: f64, t1389: f64, t25240: f64, t3964: f64, t7262: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64) {
    let t26014 = t2018 * t3951;
    let t26015 = t807 * t26014;
    let t26016 = 0.11433071498151929859e-3_f64 * t26015;
    let t26017 = t1941 * t550;
    let t26021 = t3964 * t25240 * t1389;
    let t26024 = t820 * t7262 * t843;
    (t26014, t26016, t26017, t26021, t26024)
}
