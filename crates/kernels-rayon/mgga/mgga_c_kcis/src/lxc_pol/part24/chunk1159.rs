//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1159/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1159(t20: f64, t3110: f64, t688: f64, t7592: f64, t7583: f64, t2381: f64, t26579: f64, t209: f64, t2415: f64, t705: f64, t73: f64, t9251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92235 = t3110 * t20;
    let t92236 = t688 * t92235;
    let t92237 = t92236 * t7592;
    let t92239 = t92236 * t7583;
    let t92241 = t2381 * t26579;
    let t92242 = t92241 * t7592;
    let t92247 = t209 * t73 * t9251 * t705 * t2415;
    (t92235, t92237, t92239, t92241, t92242, t92247)
}
