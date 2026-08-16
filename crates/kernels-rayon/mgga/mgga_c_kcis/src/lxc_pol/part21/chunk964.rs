//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 964/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk964(t1092: f64, t14623: f64, t1017: f64, t342: f64, t86: f64, t1130: f64, t1767: f64, t2815: f64, t9410: f64, t1662: f64, t9517: f64, t3200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14624 = t1092 * t14623;
    let t14627 = t86 * t1017 * t342;
    let t14628 = t1130 * t1767;
    let t14629 = t14628 * t2815;
    let t14630 = t9410 * t14629;
    let t14631 = t14627 * t14630;
    let t14633 = t1662 * t2815;
    let t14634 = t9517 * t14633;
    let t14635 = t3200 * t14634;
    (t14624, t14627, t14628, t14629, t14631, t14633, t14635)
}
