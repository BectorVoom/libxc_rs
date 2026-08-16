//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 684/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk684(t167: f64, t994: f64, t4951: f64, t1705: f64, t25: f64, t285: f64, t1704: f64, t330: f64, t829: f64, t2894: f64, t2909: f64, t1003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4952 = t994 * t167;
    let t4953 = t4951 * t4952;
    let t4958 = t25 * t1705;
    let t4959 = t285 * t4958;
    let t4961 = t1704 * t330;
    let t4962 = t4961 * t829;
    let t4963 = t2894 * t4962;
    let t4966 = t2909 * t1704;
    let t4967 = t4966 * t1003;
    (t4952, t4953, t4958, t4959, t4962, t4963, t4966, t4967)
}
