//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 617/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk617(t160: f64, t531: f64, t1444: f64, t740: f64, t833: f64, t1452: f64, t743: f64, t1431: f64, t733: f64, t1438: f64, t738: f64, t113: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4059 = t160 * t531;
    let t4060 = 0.15538616723388920628e-3_f64 * t4059;
    let t4061 = t740 * t1444;
    let t4062 = t4061 * t833;
    let t4073 = t743 * t1452;
    let t4081 = t733 * t1431;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    (t4059, t4060, t4061, t4062, t4073, t4081, t4089, t4093)
}
