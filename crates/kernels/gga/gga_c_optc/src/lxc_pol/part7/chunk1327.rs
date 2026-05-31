//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1327/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1327<F: Float>(t2995: F, t3012: F, t3018: F, t1057: F, t2993: F, t8679: F, t3021: F, t8582: F, t8569: F, t8577: F, t1094: F, t1102: F, t26229: F, t2916: F) -> (F, F, F, F, F) {
    let t26476 = F::cast_from(36.0_f64) * t3018 * t2995 * t3012;
    let t26479 = F::cast_from(8.0_f64) * t2993 * t1057 * t8679;
    let t26482 = F::cast_from(0.57894567559743977359e3_f64) * t8582 * t3021 * t3012;
    let t26484 = F::cast_from(0.19298189186581325786e3_f64) * t8577 * t8569;
    let t26488 = F::cast_from(0.35089340384731224426e1_f64) * t1102 * t2916 * t26229 * t1094;
    (t26476, t26479, t26482, t26484, t26488)
}
