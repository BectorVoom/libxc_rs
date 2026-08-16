//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1857/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857(t10073: f64, t2066: f64, t25390: f64, t886: f64, t7058: f64, t95730: f64, t2439: f64, t26434: f64, t887: f64, t2471: f64, t26563: f64, t10985: f64, t26576: f64) -> (f64, f64, f64, f64, f64) {
    let t95911 = t10073 * t25390 * t2066 * t886;
    let t95914 = 0.22487184191643109717e-1_f64 * t7058 * t95730;
    let t95925 = t2439 * t26434 * t887;
    let t95927 = t26563 * t2471;
    let t95930 = 0.46263278077393568556e-2_f64 * t26576 * t10985;
    (t95911, t95914, t95925, t95927, t95930)
}
