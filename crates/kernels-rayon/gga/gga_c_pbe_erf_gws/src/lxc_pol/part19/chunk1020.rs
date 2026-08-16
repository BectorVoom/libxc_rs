//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1020/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1020(t11419: f64, t8991: f64, t2118: f64, t8981: f64, t9499: f64, t824: f64, t8895: f64, t9125: f64, t3222: f64, t9607: f64, t1153: f64, t8989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11421 = t11419 * t8991 / 24.0_f64;
    let t11422 = t2118 * t8981;
    let t11423 = t9499 * t11422;
    let t11426 = t824 * t8895;
    let t11427 = t9499 * t11426;
    let t11430 = t824 * t9125;
    let t11431 = t9499 * t11430;
    let t11434 = t9607 * t3222;
    let t11435 = t1153 * t11434;
    let t11438 = t824 * t8989;
    (t11421, t11422, t11423, t11426, t11427, t11430, t11431, t11434, t11435, t11438)
}
