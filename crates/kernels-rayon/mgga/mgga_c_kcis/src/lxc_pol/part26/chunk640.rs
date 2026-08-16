//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 640/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk640(t509: f64, t7192: f64, t552: f64, t557: f64, t303: f64, t2012: f64, t5752: f64, t1464: f64, t3187: f64, t3188: f64, t6284: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7193 = t509 * t7192;
    let t7194 = t7193 * t552;
    let t7195 = t7194 * t557;
    let t7196 = t303 * t7195;
    let t7198 = t5752 * t2012;
    let t7199 = t1464 * t7198;
    let t7202 = t6284 * t8 + t3187 + t3188;
    (t7193, t7194, t7195, t7196, t7198, t7199, t7202)
}
