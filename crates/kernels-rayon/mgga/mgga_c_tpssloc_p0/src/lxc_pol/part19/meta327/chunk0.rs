//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1162/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162(t12250: f64, t40045: f64, t550: f64, t1336: f64, t2690: f64, t3788: f64, t3795: f64, t3792: f64, t67: f64, t6924: f64, t246: f64, t12156: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40148 = t40045 * t12250;
    let t40153 = t40045 * t550;
    let t40159 = t1336 * t3788 * t2690;
    let t40160 = t40159 * t3795;
    let t40162 = t40045 * t3792;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40169 = t550 * t12156;
    (t40148, t40153, t40160, t40162, t40168, t40169)
}
