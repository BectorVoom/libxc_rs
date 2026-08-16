//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2043/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2043(t12328: f64, t1333: f64, t1336: f64, t2690: f64, t3788: f64, t67: f64, t6924: f64, t246: f64, t12250: f64, t1307: f64, t39037: f64, t522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40145 = t1333 * t12328;
    let t40159 = t1336 * t3788 * t2690;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40192 = t12250 * t1307;
    let t40224 = 840.0_f64 * t39037 * t522;
    (t40145, t40159, t40167, t40168, t40192, t40224)
}
