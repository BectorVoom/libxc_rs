//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1143/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1143(t11675: f64, t19872: f64, t11678: f64, t6395: f64, t10764: f64, t26282: f64, t10882: f64, t11748: f64, t38152: f64, t7418: f64, t38149: f64, t39469: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39713 = t19872 * t11675;
    let t39715 = t6395 * t11678;
    let t39717 = t26282 * t10764;
    let t39719 = t11748 * t10882;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    (t39713, t39715, t39717, t39719, t39721, t39723)
}
