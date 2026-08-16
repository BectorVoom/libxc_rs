//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 916/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk916(t6432: f64, t978: f64, t1767: f64, t829: f64, t4566: f64, t14381: f64, t4554: f64, t3182: f64, t6555: f64, t1021: f64, t1092: f64, t4995: f64, t4999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19674 = t6432 * t978;
    let t19679 = t1767 * t829;
    let t19680 = t4566 * t19679;
    let t19681 = t14381 * t19680;
    let t19682 = t4554 * t19681;
    let t19684 = t3182 * t6555;
    let t19685 = t1021 * t19684;
    let t19686 = t1092 * t19685;
    let t19688 = t4999 * t4995;
    (t19674, t19679, t19680, t19682, t19684, t19686, t19688)
}
