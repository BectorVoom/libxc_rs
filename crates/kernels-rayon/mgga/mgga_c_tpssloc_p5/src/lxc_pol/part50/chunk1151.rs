//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1151/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1151(t31003: f64, t9231: f64, t131: f64, t31009: f64, t9239: f64, t2240: f64, t6489: f64, t79: f64, t8306: f64, t39063: f64, t31016: f64, t22642: f64, t22643: f64, t8458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t113851 = t9231 * t31003;
    let t113861 = t31009 * t131;
    let t113862 = t9239 * t113861;
    let t113869 = t2240 * t6489 * t131;
    let t113874 = t2240 * t113861;
    let t113875 = t8306 * t79;
    let t113880 = t9231 * t31009;
    let t113883 = t39063 * t31003;
    let t113888 = t9239 * t31016;
    let t113934 = 0.16449340668482264365e-1_f64 * t22642 * t22643 * t8458;
    (t113851, t113862, t113869, t113874, t113875, t113880, t113883, t113888, t113934)
}
