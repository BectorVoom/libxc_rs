//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1102/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1102(t11584: f64, t37373: f64, t37426: f64, t37427: f64, t37428: f64, t898: f64, t10929: f64, t37434: f64, t37435: f64, t10648: f64, t11583: f64, t37453: f64) -> (f64, f64, f64, f64) {
    let t39221 = t37373 * t11584;
    let t39225 = t37426 * t37427 * t898 * t37428;
    let t39229 = t37434 * t37435 * t898 * t10929;
    let t39233 = t10648 * t37453 * t11583;
    (t39221, t39225, t39229, t39233)
}
