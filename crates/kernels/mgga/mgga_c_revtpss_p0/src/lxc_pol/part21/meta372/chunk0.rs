//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1763/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1763<F: Float>(t12228: F, t3435: F, t12248: F, t3418: F, t698: F) -> (F, F, F) {
    let t12249 = t12228 * t3435;
    let t12251 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t12249;
    let t12252 = t698 * t3418;
    (t12249, t12251, t12252)
}
