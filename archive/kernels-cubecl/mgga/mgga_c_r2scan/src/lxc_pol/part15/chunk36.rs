//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 36/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk36<F: Float>(t41: F, t89: F, t61: F, t86: F) -> (F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t90 = t41 * t89;
    let t92 = F::cast_from(0.19751673498613801407e-1_f64) * t61 * t86;
    let t93 = F::ln(F::cast_from(2.0_f64));
    let t94 = F::cast_from(1.0_f64) - t93;
    let t95 = pi * pi;
    (t90, t92, t94, t95)
}
