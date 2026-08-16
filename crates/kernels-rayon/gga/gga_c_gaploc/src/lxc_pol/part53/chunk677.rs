//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 677/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk677(t3085: f64, t6508: f64, t2365: f64, t4391: f64, t2366: f64, t3116: f64, t1429: f64, t901: f64, t9302: f64, t9298: f64, t161: f64, t165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12454 = t6508 * t3085;
    let t12455 = t2365 * t12454;
    let t12456 = t4391 * t12455;
    let t12506 = t2366 * t3116;
    let t12507 = t2365 * t12506;
    let t12508 = t1429 * t12507;
    let t12510 = t9302 * t901;
    let t12512 = t9298 * t901;
    let t12526 = t161 * t165 * t3085;
    (t12454, t12455, t12456, t12506, t12507, t12508, t12510, t12512, t12526)
}
