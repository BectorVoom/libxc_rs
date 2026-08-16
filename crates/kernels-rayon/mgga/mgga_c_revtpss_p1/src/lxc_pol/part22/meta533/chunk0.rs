//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2330/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2330(t16696: f64, t5332: f64, t3720: f64, t12772: f64, t5406: f64, t3625: f64, t1248: f64, t5245: f64, t1250: f64, t1802: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17380 = t5332 * t16696;
    let t17381 = t3720 * t17380;
    let t17384 = t12772 * t5406;
    let t17386 = 0.19055119163586549765e-3_f64 * t3625 * t17384;
    let t17389 = t5245 * t1248;
    let t17390 = t17389 * t1250;
    let t17391 = t3720 * t17390;
    let t17394 = t474 * t1802;
    (t17380, t17381, t17384, t17386, t17389, t17390, t17391, t17394)
}
