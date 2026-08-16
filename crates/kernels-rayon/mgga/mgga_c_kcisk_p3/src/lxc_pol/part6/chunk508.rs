//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 508/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk508(t2178: f64, t3748: f64, t1390: f64, t470: f64, t3529: f64, t453: f64, t1336: f64, t140: f64, t3532: f64, t2181: f64, t443: f64, t1354: f64, t2059: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5610 = t3748 * t2178;
    let t5625 = t470 * t1390;
    let t5631 = t3529 * t453;
    let t5633 = t140 * t1336 * t5631;
    let t5634 = t470 * t3532;
    let t5641 = t443 * t2181;
    let t5646 = t1354 * t2059;
    (t5610, t5625, t5633, t5634, t5641, t5646)
}
