//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2011/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2011(t12020: f64, t7936: f64, t16022: f64, t20029: f64, t26224: f64, t5325: f64, t7214: f64, t7937: f64, t90493: f64, t90496: f64, t90498: f64, t90503: f64, t93306: f64, t93309: f64, t93310: f64, t93311: f64, t93333: f64, t96848: f64, t96851: f64, t96854: f64, t96857: f64, t96866: f64, t96868: f64, t96873: f64, t96878: f64) -> f64 {
    let t102466 = t12020 * t7936;
    let t102475 = -0.49348022005446793095e-1_f64 * t96848 + 0.3289868133696452873e-1_f64 * t96851 - 2.0_f64 * t16022 * t7937 + t93306 + 0.19739208802178717238e0_f64 * t96854 + t93309 + t93310 - t93311 - 0.16449340668482264365e-1_f64 * t96857 - 0.3289868133696452873e-1_f64 * t96866 + 0.38381794893125283518e-1_f64 * t96868 + 0.3289868133696452873e-1_f64 * t96873 - 12.0_f64 * t26224 * t102466 * t5325 - 2.0_f64 * t20029 * t7214 + 0.82246703342411321825e-2_f64 * t96878 - t90493 - t90496 - 0.46058153871750340221e0_f64 * t90498 - t93333 + 0.25587863262083522345e0_f64 * t90503;
    t102475
}
