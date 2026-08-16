//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 772/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk772(t7229: f64, t7364: f64, t236: f64, t3899: f64, t507: f64, t2004: f64, t7921: f64, t2007: f64, t1987: f64, t1990: f64, t1993: f64, t7920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36489 = t7229 * t7364;
    let t36504 = t507 * t236 * t3899;
    let t36508 = t7921 * t2004;
    let t36511 = t7921 * t2007;
    let t36513 = t7921 * t1987;
    let t36515 = t7921 * t1990;
    let t36520 = t1993 * t7920;
    (t36489, t36504, t36508, t36511, t36513, t36515, t36520)
}
