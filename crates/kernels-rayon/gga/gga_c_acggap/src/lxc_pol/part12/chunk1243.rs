//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1243/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1243(t32130: f64, t36495: f64, t38086: f64, t7963: f64, t7965: f64, t23688: f64, t7942: f64, t8306: f64, t33509: f64, t1264: f64, t157: f64, t2146: f64, t2147: f64, t2152: f64, t2385: f64, t33264: f64, t33266: f64, t33271: f64, t33274: f64, t33278: f64, t33281: f64, t33284: f64, t33286: f64, t33644: f64, t5079: f64, t633: f64, t7931: f64, t9427: f64) -> f64 {
    let t38415 = 0.34694512752820797848e1_f64 * t32130 * t38086 * t36495;
    let t38418 = 0.17347256376410398924e1_f64 * t7963 * t38086 * t7965;
    let t38430 = 0.17347256376410398924e1_f64 * t7942 * t8306 * t23688;
    let t38432 = t7942 * t8306 * t33509;
    let t38437 = 0.8673628188205199462e0_f64 * t33264 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t633 * t5079 * t157 - t38415 + t38418 - 0.17347256376410398924e1_f64 * t33266 - t33271 + 0.17347256376410398924e1_f64 * t33274 - 0.17347256376410398924e1_f64 * t33278 + 0.17347256376410398924e1_f64 * t33281 - t33284 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t2385 * t1264 - 0.69389025505641595696e1_f64 * t33286 - t38430 - 0.8673628188205199462e0_f64 * t38432 + 0.17347256376410398924e1_f64 * t7931 * t9427 * t33644;
    t38437
}
