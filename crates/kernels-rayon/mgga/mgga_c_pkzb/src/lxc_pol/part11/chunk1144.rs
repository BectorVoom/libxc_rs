//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1144/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1144(t2099: f64, t2945: f64, t9595: f64, t9599: f64, t2029: f64, t9539: f64, t2922: f64, t774: f64, t9278: f64, t7664: f64, t9283: f64, t5925: f64, t9613: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26457 = t2945 * t2099 * t9595;
    let t26460 = t2945 * t2099 * t9599;
    let t26494 = t9539 * t2029;
    let t26510 = t2922 * t774 * t9278;
    let t26513 = t7664 * t774 * t9283;
    let t26527 = t5925 * t9613;
    (t26457, t26460, t26494, t26510, t26513, t26527)
}
