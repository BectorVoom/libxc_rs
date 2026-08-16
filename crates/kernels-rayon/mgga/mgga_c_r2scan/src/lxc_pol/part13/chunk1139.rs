//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1139/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1139(t10760: f64, t25307: f64, t6093: f64, t19865: f64, t25314: f64, t261: f64, t3304: f64, t7239: f64, t1054: f64, t6583: f64, t7326: f64, t10799: f64, t2207: f64, t3613: f64) -> (f64, f64, f64, f64, f64) {
    let t39655 = t6093 * t10760 * t25307;
    let t39658 = t19865 * t10760 * t25314;
    let t39661 = t3304 * t261 * t7239;
    let t39664 = t6583 * t1054 * t7326;
    let t39667 = t2207 * t3613 * t10799;
    (t39655, t39658, t39661, t39664, t39667)
}
