//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1147/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1147(t14435: f64, t14482: f64, t14531: f64, t14561: f64, t1009: f64, t1014: f64, t4925: f64, t4768: f64, t978: f64, t2846: f64, t4999: f64, t2842: f64) -> (f64, f64, f64, f64, f64) {
    let t14563 = t14435 + t14482 + t14531 + t14561;
    let t14564 = t14563 * t1009;
    let t14567 = t1014 * t4925;
    let t14568 = 0.33163888888888888888e-2_f64 * t14567;
    let t14570 = t4768 * t978;
    let t14573 = t4999 * t2846;
    let t14574 = t2842 * t14573;
    (t14564, t14567, t14568, t14570, t14574)
}
