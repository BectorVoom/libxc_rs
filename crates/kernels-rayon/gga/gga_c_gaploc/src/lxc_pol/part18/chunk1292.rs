//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1292/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1292(t1858: f64, t3431: f64, t5679: f64, t7682: f64, t8792: f64, t2628: f64, t8521: f64, t2009: f64, t2021: f64, t2028: f64, t28529: f64, t33205: f64, t33206: f64, t33210: f64, t33212: f64, t33215: f64, t33218: f64, t33221: f64, t33223: f64, t33225: f64, t33228: f64, t33231: f64) -> f64 {
    let t33232 = t1858 * t3431;
    let t33238 = 0.21450293971110256002e1_f64 * t5679 * t8792 * t7682;
    let t33239 = t8521 * t2628;
    let t33240 = 0.59584149919750711116e-1_f64 * t33239;
    let t33241 = -t33205 - 0.79445533226334281486e-1_f64 * t33206 * t2028 - t33210 - t33212 + t28529 + t33215 - t33218 + t33221 - t33223 - t33225 + t33228 - t33231 - 0.71500979903700853338e0_f64 * t2021 * t33232 * t2009 - t33238 - t33240;
    t33241
}
