//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 756/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk756(t3075: f64, t996: f64, t221: f64, t346: f64, t696: f64, t345: f64, t2270: f64, t344: f64, t1003: f64, t1007: f64, t360: f64, t365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3076 = t996 * t3075;
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / 432.0_f64;
    let t3083 = t2270 * t344;
    let t3086 = t1003 * t1007;
    let t3088 = t360 * t365;
    (t3076, t3080, t3082, t3083, t3086, t3088)
}
