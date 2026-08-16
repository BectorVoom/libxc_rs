//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1270/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1270<F: Float>(t1102: F, t12478: F, t3053: F, t8697: F, t1055: F, t3018: F, t3020: F, t8679: F, t3016: F, t375: F, t3019: F, t26153: F) -> (F, F, F) {
    let t26188 = F::cast_from(0.61523382126046769581e4_f64) * t1102 * t8697 * t3053 * t12478;
    let t26192 = F::cast_from(0.64327297288604419288e2_f64) * t3018 * t8679 * t3020 * t1055;
    let t26193 = t3016 * t3016;
    let t26195 = t375 / t26193;
    let t26196 = t3019 * t3019;
    let t26197 = F::cast_from(1.0_f64) / t26196;
    let t26200 = F::cast_from(0.24954977986735470917e5_f64) * t26195 * t26153 * t26197;
    (t26188, t26192, t26200)
}
