//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1097/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1097(t38182: f64, t927: f64, t2626: f64, t503: f64, t5119: f64, t2842: f64, t37699: f64, t10698: f64, t2593: f64, t38152: f64, t7418: f64, t38149: f64, t39469: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39637 = t38182 * t927;
    let t39640 = t503 * t5119 * t2626;
    let t39642 = t37699 * t2842;
    let t39672 = t10698 * t2593;
    let t39673 = 0.25610080155860322884e0_f64 * t39672;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    (t39637, t39640, t39642, t39673, t39721, t39723)
}
