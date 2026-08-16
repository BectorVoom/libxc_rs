//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 637/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk637(t5441: f64, t6159: f64, t1369: f64, t737: f64, t1601: f64, t167: f64, t2105: f64, t25: f64, t1599: f64, t2104: f64, t531: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6160 = t6159 * t5441;
    let t6163 = t737 * t1369;
    let t6164 = t1601 * t167;
    let t6165 = t6163 * t6164;
    let t6168 = t25 * t2105;
    let t6169 = t1599 * t6168;
    let t6171 = t2104 * t531;
    let t6172 = t6171 * t833;
    (t6160, t6163, t6164, t6165, t6169, t6171, t6172)
}
