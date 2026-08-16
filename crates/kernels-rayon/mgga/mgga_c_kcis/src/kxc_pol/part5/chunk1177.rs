//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1177/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1177(t19679: f64, t4580: f64, t14381: f64, t3200: f64, t1014: f64, t6483: f64, t1133: f64, t6555: f64, t3218: f64, t1021: f64, t1092: f64, t1121: f64) -> (f64, f64, f64, f64, f64) {
    let t19750 = t4580 * t19679;
    let t19751 = t14381 * t19750;
    let t19752 = t3200 * t19751;
    let t19754 = t1014 * t6483;
    let t19756 = t6555 * t1133;
    let t19757 = t3218 * t19756;
    let t19758 = t1021 * t19757;
    let t19759 = t1092 * t19758;
    let t19763 = t6555 * t1121;
    (t19752, t19754, t19756, t19759, t19763)
}
