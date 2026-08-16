//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 177/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk177(t25: f64, t28: f64, t17: f64, t522: f64, t182: f64, t521: f64, t514: f64, t194: f64, t517: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t523 = t17 * t522;
    let t525 = 0.19751673498613801407e-1_f64 * t521 * t182;
    let t526 = t514 * t514;
    let t527 = piecewise3(t26, t194, t526);
    let t528 = t517 * t517;
    let t529 = piecewise3(t29, t194, t528);
    let t531 = t527 / 2.0_f64 + t529 / 2.0_f64;
    (t523, t525, t526, t528, t531)
}
