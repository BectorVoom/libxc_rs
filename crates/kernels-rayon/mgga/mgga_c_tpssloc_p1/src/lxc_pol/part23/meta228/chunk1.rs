//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 878/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk878(t3865: f64, t5234: f64, t12189: f64, t1811: f64, t1815: f64, t3862: f64, t3802: f64, t3788: f64, t836: f64, t1336: f64, t1834: f64, t3787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16336 = t5234 * t3865;
    let t16341 = t12189 * t1811;
    let t16350 = t1815 * t3862;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    let t16398 = t1336 * t16397;
    let t16428 = t3787 * t1834;
    (t16336, t16341, t16350, t16394, t16398, t16428)
}
