//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 756/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk756(t4811: f64, t4818: f64, t4817: f64, t5069: f64, t1869: f64, t1894: f64, t4797: f64, t1801: f64, t5062: f64, t1755: f64, t695: f64, t1060: f64, t4972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11663 = t4811 * t4818;
    let t11668 = t4817 * t5069;
    let t11669 = t1869 * t11668;
    let t11671 = t4797 * t1894;
    let t11672 = t1801 * t11671;
    let t11673 = t5062 * t11672;
    let t11674 = t1869 * t11673;
    let t11676 = t1755 * t695;
    let t11677 = t1060 * t4972;
    (t11663, t11669, t11671, t11674, t11676, t11677)
}
