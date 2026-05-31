//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3228/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3228<F: Float>(t40121: F, t50058: F, t40127: F, t40132: F, t18263: F, t2414: F, t40207: F, t6002: F, t40139: F, t50084: F, t14353: F, t14365: F, t18871: F, t2403: F, t40131: F, t40137: F, t4433: F, t4541: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61214 = F::cast_from(0.20779030926817756511e3_f64) * t40121;
    let t61215 = F::cast_from(16.0_f64) * t50058;
    let t61219 = F::cast_from(0.24415263074675393405e-3_f64) * t40127;
    let t61220 = F::cast_from(0.11696447245269292414e1_f64) * t40132;
    let t61222 = F::cast_from(4.0_f64) * t18263 * t2414;
    let t61224 = F::cast_from(12.0_f64) * t40207 * t6002;
    let t61225 = F::cast_from(8.0_f64) * t40139;
    let t61229 = F::cast_from(8.0_f64) * t50084;
    let t61230 = F::cast_from(24.0_f64) * t14353 * t4433 * t4541 + F::cast_from(12.0_f64) * t14365 * t18871 * t2403 - t40131 - t40137 + t61214 + t61215 + t61219 - t61220 + t61222 + t61224 + t61225 + t61229;
    (t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61230)
}
