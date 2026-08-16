//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1877/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1877(t25365: f64, t26544: f64, t93190: f64, t95726: f64, t2435: f64, t26560: f64, t10073: f64, t2066: f64, t25390: f64, t886: f64, t7058: f64, t95730: f64) -> (f64, f64, f64, f64, f64) {
    let t95900 = t25365 * t26544;
    let t95902 = t93190 * t95726;
    let t95905 = t2435 * t26560;
    let t95911 = t10073 * t25390 * t2066 * t886;
    let t95914 = 0.22487184191643109717e-1_f64 * t7058 * t95730;
    (t95900, t95902, t95905, t95911, t95914)
}
