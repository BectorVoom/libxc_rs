//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 866/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk866(t1498: f64, t5752: f64, t1464: f64, t1494: f64, t1928: f64, t1497: f64, t1395: f64, t2012: f64, t3738: f64, t2013: f64, t3728: f64, t2003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5753 = t5752 * t1498;
    let t5754 = t1464 * t5753;
    let t5756 = t1928 * t1494;
    let t5757 = t5756 * t1497;
    let t5758 = t1395 * t5757;
    let t5759 = t1464 * t5758;
    let t5761 = t3738 * t2012;
    let t5762 = t1464 * t5761;
    let t5764 = t3728 * t2013;
    let t5766 = t3728 * t2003;
    (t5753, t5754, t5756, t5757, t5758, t5759, t5761, t5762, t5764, t5766)
}
