//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 968/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk968(t2080: f64, t4332: f64, t2084: f64, t3919: f64, t2072: f64, t4330: f64, t16144: f64, t16048: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11455: f64, t11457: f64, t11460: f64, t16050: f64, t16062: f64, t16088: f64) -> (f64, f64, f64, f64, f64) {
    let t17828 = t2080 * t4332;
    let t17831 = t2084 * t3919;
    let t17834 = t2072 * t4330;
    let t17847 = 0.27785333333333333334e0_f64 * t16144;
    let t17856 = 0.22954444444444444444e0_f64 * t16048;
    let t17861 = -0.45908888888888888888e0_f64 * t11409 + 0.11477222222222222222e0_f64 * t11411 - 0.34431666666666666666e0_f64 * t11413 + 0.17215833333333333333e0_f64 * t11415 + 0.103295e1_f64 * t16088 + 0.20659e1_f64 * t16062 + t17856 - 0.68863333333333333333e0_f64 * t16050 - 0.23154444444444444444e0_f64 * t11455 + 0.69463333333333333333e-1_f64 * t11457 + 0.23154444444444444444e-1_f64 * t11460;
    (t17828, t17831, t17834, t17847, t17861)
}
