//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 853/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk853(t16131: f64, t3781: f64, t3820: f64, t5481: f64, t1319: f64, t3809: f64, t5513: f64, t1330: f64, t16078: f64, t4714: f64, t5567: f64, t659: f64) -> (f64, f64, f64, f64, f64) {
    let t16132 = t16131 * t3781;
    let t16134 = t3820 * t5481;
    let t16135 = t16134 * t1319;
    let t16137 = t5513 * t3809;
    let t16141 = t1330 * t16078;
    let t16142 = t4714 * t16141;
    let t16144 = t659 * t5567;
    (t16132, t16135, t16137, t16142, t16144)
}
