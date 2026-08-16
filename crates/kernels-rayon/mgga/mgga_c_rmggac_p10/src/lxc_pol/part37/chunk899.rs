//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 899/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk899(t4669: f64, t75200: f64, t25820: f64, t75956: f64, t27101: f64, t75962: f64, t41407: f64, t649: f64, t8982: f64, t40928: f64, t8963: f64, t40932: f64, t8937: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76140 = t4669 * t75200;
    let t76141 = 0.23948483403727617128e0_f64 * t76140;
    let t76143 = t25820 * t75956;
    let t76145 = t27101 * t75962;
    let t76148 = t41407 * t649 * t8982;
    let t76151 = t40928 * t649 * t8963;
    let t76154 = t40932 * t649 * t8937;
    (t76141, t76143, t76145, t76148, t76151, t76154)
}
