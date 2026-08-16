//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1093/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1093(t10929: f64, t37434: f64, t37435: f64, t898: f64, t10648: f64, t11583: f64, t37453: f64, t10992: f64, t11563: f64, t2315: f64, t3446: f64, t10649: f64, t2482: f64, t58: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t39229 = t37434 * t37435 * t898 * t10929;
    let t39233 = t10648 * t37453 * t11583;
    let t39239 = t3446 * t10992 * t11563 * t2315;
    let t39244 = t10648 * t10649 * t58 * t2482 * t597;
    (t39229, t39233, t39239, t39244)
}
