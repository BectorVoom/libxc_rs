//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2468/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2468(t1086: f64, t11200: f64, t3090: f64, t11671: f64, t11926: f64, t1045: f64, t2862: f64, t999: f64, t3075: f64, t606: f64, t16565: f64, t994: f64) -> (f64, f64, f64, f64, f64) {
    let t43291 = t11200 * t1086 * t3090;
    let t43297 = t11926 * t11671;
    let t43301 = t1045 * t2862 * t999;
    let t43313 = t606 * t3075;
    let t43341 = t994 * t16565;
    (t43291, t43297, t43301, t43313, t43341)
}
