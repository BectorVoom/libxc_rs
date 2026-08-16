//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1190/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1190(t2323: f64, t6451: f64, t4395: f64, t6638: f64, t4408: f64, t6670: f64, t822: f64, t6680: f64, t4413: f64, t6673: f64, t6672: f64, t6501: f64, t6548: f64) -> (f64, f64, f64, f64, f64) {
    let t21146 = t2323 * t6451;
    let t21148 = t4395 * t6638;
    let t21152 = t4408 * t6670;
    let t21153 = t822 * t21152;
    let t21155 = t21153 * t6680 / 12.0_f64;
    let t21156 = t4413 * t6673;
    let t21157 = t6672 * t21156;
    let t21158 = 7.0_f64 / 6.0_f64 * t21157;
    let t21159 = t6501 * t6548;
    (t21146, t21148, t21155, t21158, t21159)
}
