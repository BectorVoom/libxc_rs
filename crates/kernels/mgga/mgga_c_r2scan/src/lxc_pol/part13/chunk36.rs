//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 36/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk36<F: Float>(t41: F, t89: F, t61: F, t86: F) -> (F, F, F, F) {
    let t90 = t41 * t89;
    let t92 = F::new(0.19751673498613801407e-1) * t61 * t86;
    let t93 = f64::ln(F::new(2.0));
    let t94 = F::new(1.0) - t93;
    let t95 = M_PI * M_PI;
    (t90, t92, t94, t95)
}
