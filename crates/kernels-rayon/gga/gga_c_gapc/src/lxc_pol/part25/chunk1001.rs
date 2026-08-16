//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1001/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1001(t11522: f64, t5541: f64, t8677: f64, t5462: f64, t8681: f64, t3670: f64, t620: f64, t190: f64, t8448: f64, t1: f64, t116: f64, t3703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11523 = t5541 * t11522;
    let t11524 = t11523 * t8677;
    let t11526 = t5462 * t11522;
    let t11527 = t11526 * t8681;
    let t11529 = t3670 * t620;
    let t11532 = t190 * t8448;
    let t11533 = t11532 * t1;
    let t11534 = t116 * t11533;
    let t11535 = t11534 * t3703;
    (t11523, t11524, t11526, t11527, t11529, t11533, t11534, t11535)
}
