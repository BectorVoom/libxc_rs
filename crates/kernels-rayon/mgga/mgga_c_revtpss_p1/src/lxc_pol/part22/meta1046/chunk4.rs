//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3675/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3675(t3451: f64, t6481: f64, t1188: f64, t12423: f64, t12470: f64, t12486: f64, t12511: f64, t16966: f64, t17085: f64, t1745: f64, t20606: f64, t20609: f64, t20612: f64, t20671: f64, t3452: f64, t3453: f64, t3454: f64, t3471: f64, t3477: f64, t3479: f64, t3496: f64, t3497: f64, t3515: f64, t45197: f64, t58005: f64, t6487: f64, t6506: f64, t6535: f64, t68795: f64, t69094: f64, t69097: f64, t69099: f64, t69101: f64, t69103: f64, t69105: f64, t69107: f64, t69367: f64) -> f64 {
    let t69488 = t6481 * t3451;
    let t69500 = 0.4138081033541872024e4_f64 * t58005 * t16966 + 12.0_f64 * t12423 * t20606 + 6.0_f64 * t3477 * t6487 * t3471 + 0.11579025239058625248e4_f64 * t12470 * t6506 * t3453 - 8.0_f64 * t12511 * t20609 - 4.0_f64 * t3452 * t1745 * t17085 - 0.38596750796862084162e3_f64 * t45197 * t20612 - t69094 + 0.64327917994770140268e2_f64 * t3477 * t69367 * t3479 - 2.0_f64 * t69488 * t3454 + t69097 - 0.23392894490538584828e1_f64 * t3496 * t68795 * t1188 - t69099 - t69101 + t69103 - t69105 - t69107 - 0.11696447245269292414e1_f64 * t3496 * t6535 * t3515 - 0.10389515463408878255e3_f64 * t12486 * t20671 * t3497;
    t69500
}
