//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 283/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk283(t154: f64, t824: f64, t907: f64, t395: f64, t748: f64) -> (f64, f64, f64, f64) {
    let t909 = t154 * t907 * t824;
    let t912 = t395 * t395;
    let t913 = 1.0_f64 / t912;
    let t914 = t748 * t913;
    (t909, t912, t913, t914)
}
