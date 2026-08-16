//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 993/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk993(t13565: f64, t30396: f64, t1234: f64, t1264: f64, t13588: f64, t13680: f64, t20567: f64, t2129: f64, t26095: f64, t30308: f64, t30319: f64, t30381: f64, t30384: f64, t30388: f64, t30393: f64, t374: f64, t45: f64, t6035: f64, t6095: f64, t7960: f64, t7963: f64, t7999: f64) -> f64 {
    let t30397 = t30396 * t13565;
    let t30402 = 0.19751789702565206229e-1_f64 * t45 * t30308 * t374 + 3.0_f64 * t26095 * t2129 + 3.0_f64 * t6035 * t7960 + 0.48245472966453314466e2_f64 * t20567 * t7963 - 0.96490945932906628932e2_f64 * t13588 * t30319 + 1.0_f64 * t1234 * t30381 + 0.51725014705706168417e3_f64 * t13680 * t30384 + 0.35089340384731224426e1_f64 * t1264 * t30388 - 0.35089340384731224426e1_f64 * t1264 * t30393 - 0.1025389702100779493e4_f64 * t1264 * t30397 - 0.51947267698127589899e2_f64 * t6095 * t7999;
    t30402
}
