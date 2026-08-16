//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2633/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2633(t13768: f64, t13902: f64, t13907: f64, t1877: f64, t22229: f64, t225: f64, t4045: f64, t4053: f64, t48220: f64, t48245: f64, t48257: f64, t48272: f64, t48289: f64, t48309: f64, t48321: f64, t48337: f64, t48347: f64, t48436: f64, t541: f64, t543: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t73: f64, t9400: f64, t9881: f64, t9884: f64, t9887: f64, t9984: f64) -> f64 {
    let t48438 = (9.0_f64 * t5644 * t4053 + 180.0_f64 * t13902 * t13907 - 36.0_f64 * t4045 * t73 * t5652 - (t48220 + t48245 + t48257 + t48272 + t48289 + t48309 + t48321 + t48337) * t225 * t541 + 60.0_f64 * t1877 * t9881 - 36.0_f64 * t22229 * t9884 - 360.0_f64 * t5650 * t48347 * t9400 + 180.0_f64 * t5650 * t13768 * t9984 + 9.0_f64 * t4045 * t5655 + 3.0_f64 * t1877 * t9887 + t48436) * t543;
    t48438
}
