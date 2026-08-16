//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 632/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk632(t5556: f64, t567: f64, t564: f64, t1390: f64, t470: f64, t3529: f64, t453: f64, t1336: f64, t140: f64, t3532: f64, t5: f64, t969: f64) -> (f64, f64, f64, f64, f64) {
    let t5557 = t567 * t5556;
    let t5558 = t564 * t5557;
    let t5625 = t470 * t1390;
    let t5631 = t3529 * t453;
    let t5633 = t140 * t1336 * t5631;
    let t5634 = t470 * t3532;
    let t5680 = t5 * t969;
    (t5558, t5625, t5633, t5634, t5680)
}
