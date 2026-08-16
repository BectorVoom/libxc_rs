//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3702/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3702<F: Float>(t44865: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> F {
    let t70158 = F::cast_from(0.59266666666666666668e-1_f64) * t68253 + F::cast_from(0.65851851851851851853e-2_f64) * t68255 - F::cast_from(0.43901234567901234569e-2_f64) * t68257 + t44865 - F::cast_from(0.10975308641975308642e-1_f64) * t68262 + F::cast_from(0.16462962962962962963e-1_f64) * t68267 + F::cast_from(0.35560000000000000001e0_f64) * t68271 + F::cast_from(0.59266666666666666668e-1_f64) * t68275 - F::cast_from(0.19755555555555555556e-1_f64) * t68277 - F::cast_from(0.19755555555555555556e-1_f64) * t68282 - F::cast_from(0.9877777777777777778e-2_f64) * t68287 - F::cast_from(0.59266666666666666668e-1_f64) * t68292;
    t70158
}
