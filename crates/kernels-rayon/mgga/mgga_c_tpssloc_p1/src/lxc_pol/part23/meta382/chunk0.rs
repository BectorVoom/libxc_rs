//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1184/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1184(t1667: f64, t9709: f64, t11274: f64, t1657: f64, t11189: f64, t11282: f64, t1687: f64, t11419: f64, t1675: f64, t11349: f64, t11292: f64, t1714: f64, t44583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50846 = t9709 * t1667;
    let t51120 = t1657 * t11274;
    let t51249 = t1657 * t11189;
    let t51376 = t1687 * t11282;
    let t51427 = t1675 * t11419;
    let t51604 = t1675 * t11349;
    let t51680 = t1687 * t11292;
    let t51968 = t44583 * t1714;
    (t50846, t51120, t51249, t51376, t51427, t51604, t51680, t51968)
}
