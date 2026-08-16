//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 661/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk661(t24: f64, t586: f64, t9007: f64, t1775: f64, t2103: f64, t2106: f64, t2: f64, t9114: f64, t9050: f64, t2097: f64, t8315: f64, t3499: f64, t7807: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9186 = t24 * t586 * t9007;
    let t9188 = t1775 * t2103;
    let t9190 = t1775 * t2106;
    let t9192 = t9114 * t2;
    let t9193 = t9192 * t9050;
    let t9196 = t2097 * t8315;
    let t9199 = t3499 * t7807;
    (t9186, t9188, t9190, t9192, t9193, t9196, t9199)
}
