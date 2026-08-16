//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2217/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2217(t22633: f64, t26421: f64, t3856: f64, t6976: f64, t12267: f64, t1336: f64, t22873: f64, t5287: f64, t7745: f64, t81069: f64, t81073: f64, t81075: f64, t81076: f64, t81083: f64, t81099: f64, t90903: f64, t90907: f64, t90910: f64, t90913: f64, t90917: f64, t90921: f64, t90925: f64, t90929: f64) -> f64 {
    let t90933 = t22633 * t6976 * t26421 * t3856;
    let t90939 = t90903 + 0.3289868133696452873e-1_f64 * t90907 + 0.3289868133696452873e-1_f64 * t90910 - t90913 - 0.9869604401089358619e-1_f64 * t90917 + 0.49348022005446793095e-1_f64 * t90921 - 0.41123351671205660912e-2_f64 * t81069 - t81073 - t81075 + 0.52089578783527170488e-1_f64 * t81076 - t90925 + 0.16449340668482264365e-1_f64 * t81083 + 0.19190897446562641759e-1_f64 * t81099 - 0.82246703342411321825e-2_f64 * t90929 + 0.16449340668482264365e-1_f64 * t90933 - 2.0_f64 * t1336 * t22873 * t5287 - t12267 * t7745;
    t90939
}
