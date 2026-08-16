//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 475/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk475(t44: f64, t2: f64, t898: f64, t464: f64, t1361: f64, t889: f64, t35: f64, t48: f64, t1216: f64, t415: f64, t1368: f64, t893: f64, t53: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t2463 = t898 * t2;
    let t2464 = t2463 * t464;
    let t2465 = 0.18311447306006545054e-3_f64 * t2464;
    let t2466 = t1361 * t889;
    let t2469 = t48 * t35;
    let t2473 = piecewise3(t45, 0.0_f64, 4.0_f64 / 9.0_f64 * t2466 * t415 + 8.0_f64 / 3.0_f64 * t2469 * t1216);
    let t2474 = t1368 * t893;
    let t2477 = t53 * t35;
    (t2463, t2464, t2465, t2466, t2473, t2474, t2477)
}
