//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1406/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1406(t609: f64, t4425: f64, t7421: f64, t1599: f64, t6141: f64, t6148: f64, t23024: f64, t1608: f64, t286: f64, t25: f64, t7493: f64, t7430: f64, t6168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t23191 = t4425 * t7421;
    let t23192 = t1599 * t23191;
    let t23194 = t6141 * t6148;
    let t23198 = piecewise3(t614, t23024, -t23024);
    let t23199 = t1608 * t23198;
    let t23200 = t286 * t23199;
    let t23207 = t25 * t7493;
    let t23208 = t1599 * t23207;
    let t23210 = t25 * t7430;
    let t23211 = t1599 * t23210;
    let t23213 = t6141 * t6168;
    (t23192, t23194, t23200, t23208, t23211, t23213)
}
