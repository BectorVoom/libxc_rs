//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2579/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2579(t14795: f64, t699: f64, t14798: f64, t2403: f64, t4772: f64, t14792: f64, t11274: f64, t1657: f64, t3263: f64, t4737: f64, t11189: f64, t1147: f64, t14933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51041 = t699 * t14795;
    let t51043 = t699 * t14798;
    let t51051 = t2403 * t4772;
    let t51053 = t699 * t14792;
    let t51120 = t1657 * t11274;
    let t51246 = t4737 * t3263;
    let t51249 = t1657 * t11189;
    let t51366 = t14933 * t1147;
    (t51041, t51043, t51051, t51053, t51120, t51246, t51249, t51366)
}
