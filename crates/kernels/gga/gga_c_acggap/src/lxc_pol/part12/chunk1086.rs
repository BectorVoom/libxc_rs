//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1086/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1086<F: Float>(t32130: F, t36495: F, t38086: F, t7963: F, t7965: F, t23688: F, t7942: F, t8306: F, t33509: F, t1264: F, t157: F, t2146: F, t2147: F, t2152: F, t2385: F, t33264: F, t33266: F, t33271: F, t33274: F, t33278: F, t33281: F, t33284: F, t33286: F, t33644: F, t5079: F, t633: F, t7931: F, t9427: F) -> (F,) {
    let t38415 = 0.34694512752820797848e1 * t32130 * t38086 * t36495;
    let t38418 = 0.17347256376410398924e1 * t7963 * t38086 * t7965;
    let t38430 = 0.17347256376410398924e1 * t7942 * t8306 * t23688;
    let t38432 = t7942 * t8306 * t33509;
    let t38437 = 0.8673628188205199462e0 * t33264 + 0.4336814094102599731e0 * t2146 * t2152 * t633 * t5079 * t157 - t38415 + t38418 - 0.17347256376410398924e1 * t33266 - t33271 + 0.17347256376410398924e1 * t33274 - 0.17347256376410398924e1 * t33278 + 0.17347256376410398924e1 * t33281 - t33284 + 0.8673628188205199462e0 * t2146 * t2147 * t2385 * t1264 - 0.69389025505641595696e1 * t33286 - t38430 - 0.8673628188205199462e0 * t38432 + 0.17347256376410398924e1 * t7931 * t9427 * t33644;
    (t38437,)
}
