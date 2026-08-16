//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1211/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1211(t7580: f64, t92174: f64, t26597: f64, t26623: f64, t700: f64, t9251: f64, t2387: f64, t26620: f64, t7589: f64, t209: f64, t2403: f64, t2389: f64, t2404: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92175 = t7580 * t92174;
    let t92177 = t26597 * t26623;
    let t92179 = t9251 * t700;
    let t92181 = t26620 * t92179 * t2387;
    let t92182 = t7589 * t92181;
    let t92184 = t209 * t2403;
    let t92187 = t92184 * t2389 * t2404 * t705;
    (t92175, t92177, t92181, t92182, t92184, t92187)
}
