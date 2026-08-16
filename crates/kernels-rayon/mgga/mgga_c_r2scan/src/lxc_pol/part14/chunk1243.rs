//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1243/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1243(t1018: f64, t11239: f64, t1125: f64, t12267: f64, t12271: f64, t12273: f64, t1305: f64, t1306: f64, t1307: f64, t1308: f64, t2405: f64, t330: f64, t3517: f64, t3740: f64, t3742: f64, t41854: f64, t41917: f64, t837: f64, t838: f64, t8420: f64) -> f64 {
    let t41940 = (t41854 + t41917) * t330 + 2.0_f64 * t12267 * t837 * t330 + t3740 * t1305 * t330 + t3740 * t1307 * t330 + t11239 * t1018 * t330 + 2.0_f64 * t3517 * t2405 * t330 + 2.0_f64 * t12271 * t838 + t1125 * t8420 * t330 + 2.0_f64 * t12273 * t838 + t3742 * t1306 + t3742 * t1308;
    t41940
}
