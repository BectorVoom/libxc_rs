//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 941/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk941(t3332: f64, t6402: f64, t2147: f64, t3316: f64, t776: f64, t2228: f64, t57: f64, t2116: f64, t3320: f64, t560: f64, t2201: f64, t3319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10836 = t3332 * t6402;
    let t10837 = t2147 * t10836;
    let t10839 = t776 * t3316;
    let t10840 = 0.23115257973478049502e0_f64 * t10839;
    let t10841 = t2228 * t57;
    let t10842 = t10841 * t2116;
    let t10844 = t3320 * t560;
    let t10846 = t2201 * t3319 * t10844;
    (t10836, t10837, t10839, t10840, t10841, t10842, t10844, t10846)
}
