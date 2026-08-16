//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1186/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1186(t10760: f64, t19877: f64, t29467: f64, t29731: f64, t6093: f64, t11724: f64, t26278: f64, t11675: f64, t26282: f64, t12550: f64, t2207: f64, t3328: f64) -> (f64, f64, f64, f64, f64) {
    let t43178 = t19877 * t10760 * t29467;
    let t43181 = t6093 * t10760 * t29731;
    let t43183 = t26278 * t11724;
    let t43185 = t26282 * t11675;
    let t43188 = t2207 * t12550 * t3328;
    (t43178, t43181, t43183, t43185, t43188)
}
