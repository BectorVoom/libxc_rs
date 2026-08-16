//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 756/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk756(t285: f64, t9613: f64, t3030: f64, t961: f64, t273: f64, t3033: f64, t2985: f64, t926: f64, t257: f64, t2984: f64, t244: f64, t2323: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9614 = t285 * t9613;
    let t9630 = 1.0_f64 / t3030 / t961;
    let t9634 = 1.0_f64 / t3033 / t273;
    let t9655 = t926 * t2985;
    let t9659 = 1.0_f64 / t2984 / t257;
    let t9660 = t244 * t9659;
    let t9691 = t2323 * t923;
    (t9614, t9630, t9634, t9655, t9660, t9691)
}
