//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1220/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1220(t10138: f64, t2140: f64, t688: f64, t209: f64, t706: f64, t7589: f64, t9203: f64, t9204: f64, t26576: f64, t26597: f64, t26579: f64, t9262: f64) -> (f64, f64, f64, f64) {
    let t92294 = t688 * t10138 * t2140;
    let t92300 = t7589 * t209 * t9203 * t706 * t9204;
    let t92302 = t26597 * t26576;
    let t92305 = t9262 * t26579 * t26576;
    (t92294, t92300, t92302, t92305)
}
