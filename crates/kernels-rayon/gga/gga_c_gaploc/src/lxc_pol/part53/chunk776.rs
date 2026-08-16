//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 776/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk776(t12000: f64, t599: f64, t203: f64, t1: f64, t38285: f64, t544: f64, t1564: f64, t1359: f64, t3689: f64, t12064: f64, t540: f64, t106: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38392 = t599 * t12000;
    let t38413 = t203 * t12000;
    let t38486 = t544 * t38285 * t1;
    let t38613 = t1564 * t12000;
    let t38674 = t1359 * t3689;
    let t38688 = t12064 * t540;
    let t38759 = t12000 * t1 * t106 * t192;
    (t38392, t38413, t38486, t38613, t38674, t38688, t38759)
}
