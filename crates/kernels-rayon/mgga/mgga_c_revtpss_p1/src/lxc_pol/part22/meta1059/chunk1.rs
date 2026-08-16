//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3764/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764(t1261: f64, t20272: f64, t247: f64, t3634: f64, t3584: f64, t6573: f64, t12916: f64, t20951: f64, t5340: f64, t17170: f64, t1774: f64, t17396: f64, t17620: f64) -> (f64, f64, f64, f64, f64) {
    let t71827 = t1261 * t247 * t3634 * t20272;
    let t71839 = t6573 * t3584;
    let t71845 = t5340 * t12916 * t20951;
    let t71854 = t1774 * t17170;
    let t71859 = t17396 * t17620;
    (t71827, t71839, t71845, t71854, t71859)
}
