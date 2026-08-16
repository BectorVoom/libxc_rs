//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1127/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1127(t10760: f64, t19877: f64, t25562: f64, t261: f64, t3304: f64, t7233: f64, t38182: f64, t927: f64, t2626: f64, t503: f64, t5119: f64, t2842: f64, t37699: f64) -> (f64, f64, f64, f64, f64) {
    let t39632 = t19877 * t10760 * t25562;
    let t39635 = t3304 * t261 * t7233;
    let t39637 = t38182 * t927;
    let t39640 = t503 * t5119 * t2626;
    let t39642 = t37699 * t2842;
    (t39632, t39635, t39637, t39640, t39642)
}
