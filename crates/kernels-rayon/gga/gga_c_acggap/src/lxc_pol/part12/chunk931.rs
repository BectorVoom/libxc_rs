//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 931/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk931(t31142: f64, t7437: f64, t128: f64, t576: f64, t7475: f64, t1108: f64, t7736: f64, t1967: f64, t7705: f64, t1988: f64, t7763: f64, t7767: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31143 = t31142 * t7437;
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    let t31162 = t1967 * t7705;
    let t31164 = t1988 * t7763;
    let t31166 = t1988 * t7767;
    (t31143, t31146, t31160, t31162, t31164, t31166)
}
