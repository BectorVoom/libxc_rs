//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2016/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2016(t90912: f64, t1352: f64, t24103: f64, t3773: f64, t5234: f64, t5344: f64, t7934: f64, t81069: f64, t81076: f64, t81080: f64, t81083: f64, t81099: f64, t84480: f64, t84481: f64, t90907: f64, t90910: f64, t90917: f64, t90921: f64, t90929: f64, t90933: f64, t93505: f64) -> f64 {
    let t93572 = 0.15352717957250113407e0_f64 * t90912;
    let t93587 = 0.6579736267392905746e-1_f64 * t90907 + 0.6579736267392905746e-1_f64 * t90910 - t93572 - 0.19739208802178717238e0_f64 * t90917 + 0.9869604401089358619e-1_f64 * t90921 - 0.82246703342411321825e-2_f64 * t81069 - t84480 - t84481 + 0.10417915756705434098e0_f64 * t81076 + t3773 * t7934 - 0.20835831513410868196e0_f64 * t81080 + 0.3289868133696452873e-1_f64 * t81083 + 0.38381794893125283518e-1_f64 * t81099 - 2.0_f64 * t5344 * t93505 * t1352 - t5234 * t24103 - 0.16449340668482264365e-1_f64 * t90929 + 0.3289868133696452873e-1_f64 * t90933;
    t93587
}
