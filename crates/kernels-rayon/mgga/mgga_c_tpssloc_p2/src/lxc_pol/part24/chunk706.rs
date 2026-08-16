//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 706/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk706(t205: f64, t5194: f64, t242: f64, t3788: f64, t1336: f64, t557: f64, t67: f64, t246: f64, t546: f64, t68: f64, t3787: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5195 = t205 * t5194;
    let t5245 = t3788 * t242;
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    let t5278 = t546 * t68;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    (t5195, t5246, t5247, t5248, t5278, t5334)
}
