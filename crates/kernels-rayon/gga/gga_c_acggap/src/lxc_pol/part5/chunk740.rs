//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 740/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk740(t43: f64, t5474: f64, t85: f64, t4030: f64, t2635: f64, t2835: f64, t1690: f64, t2898: f64, t1694: f64, t817: f64, t1281: f64, t234: f64, t292: f64, t5455: f64, t822: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t5475 = t5474 * t85;
    let t5476 = 0.19751673498613801407e-1_f64 * t5475;
    let t5477 = 0.48830526149350786811e-3_f64 * t4030;
    let t5478 = 12.0_f64 * t2635;
    let t5479 = 0.11696447245269292414e1_f64 * t2835;
    let t5481 = t2898 * t1690;
    let t5486 = t817 * t1694;
    let t5492 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t5481 * t234 - 8.0_f64 / 9.0_f64 * t1281 * t822 - 2.0_f64 / 9.0_f64 * t5486 * t234 + 2.0_f64 / 3.0_f64 * t292 * t5455);
    (t5475, t5476, t5477, t5478, t5479, t5481, t5486, t5492)
}
