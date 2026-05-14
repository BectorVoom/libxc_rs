//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 467/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk467<F: Float>(t44: F, t51: F, t2: F, t898: F, t464: F, t1361: F, t889: F, t35: F, t48: F, t1216: F, t415: F, t1368: F, t893: F, t53: F, t419: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2463 = t898 * t2;
    let t2464 = t2463 * t464;
    let t2465 = 0.18311447306006545054e-3 * t2464;
    let t2466 = t1361 * t889;
    let t2469 = t48 * t35;
    let t2473 = piecewise3(t45, 0.0, 4.0 / 9.0 * t2466 * t415 + 8.0 / 3.0 * t2469 * t1216);
    let t2474 = t1368 * t893;
    let t2477 = t53 * t35;
    let t2481 = piecewise3(t52, 0.0, 4.0 / 9.0 * t2474 * t419 - 8.0 / 3.0 * t2477 * t1216);
    let t2482 = t2473 + t2481;
    (t2463, t2464, t2465, t2466, t2474, t2482)
}
