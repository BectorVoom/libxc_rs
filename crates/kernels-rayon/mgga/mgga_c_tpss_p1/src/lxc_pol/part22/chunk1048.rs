//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1048/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1048(t11399: f64, t903: f64, t2613: f64, t3886: f64, t1448: f64, t8752: f64, t2595: f64, t1449: f64, t11240: f64, t11242: f64, t11379: f64, t11383: f64, t11390: f64, t11393: f64, t11396: f64, t2575: f64, t2594: f64, t2619: f64, t3865: f64, t3887: f64, t8888: f64, t8906: f64, t8912: f64, t8915: f64, t8922: f64) -> f64 {
    let t11400 = t11399 * t903;
    let t11403 = t3886 * t2613;
    let t11406 = t1448 * t8752;
    let t11407 = t11406 * t2595;
    let t11410 = t1449 * t2595;
    let t11413 = 0.32163958997385070134e2_f64 * t2575 * t11379 + 0.2069040516770936012e4_f64 * t8888 * t11383 - 0.23392894490538584828e1_f64 * t8906 * t3865 + 0.34631718211362927518e2_f64 * t8912 * t3887 - 0.23392894490538584828e1_f64 * t2594 * t11390 - 0.11696447245269292414e1_f64 * t2594 * t11393 - 0.10389515463408878255e3_f64 * t8915 * t11396 + 0.34631718211362927518e2_f64 * t2619 * t11400 + 0.17315859105681463759e2_f64 * t2619 * t11403 + 0.10254018858216406658e4_f64 * t8922 * t11407 + 0.35089341735807877242e1_f64 * t2619 * t11410 + t11240 - t11242;
    t11413
}
