//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1084/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1084(t3436: f64, t6876: f64, t122: f64, t158: f64, t166: f64, t2315: f64, t3434: f64, t10655: f64, t10946: f64, t10810: f64, t3429: f64, t3457: f64) -> (f64, f64, f64) {
    let t38317 = t3436 * t6876;
    let t38322 = t3434 * t38317 * t158 * t166 * t2315 * t122;
    let t38336 = t10655 * t10946;
    let t38339 = t3429 * t10810 * t3457;
    (t38322, t38336, t38339)
}
