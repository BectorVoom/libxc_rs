//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 70/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk70<F: Float>(t158: F, t190: F, t157: F, t162: F, t187: F) -> (F, F, F, F) {
    let t191 = t158 * t190;
    let t192 = t157 * t162;
    let t194 = F::cast_from(0.19751673498613801407e-1_f64) * t192 * t187;
    let t195 = F::ln(F::cast_from(2.0_f64));
    let t196 = F::cast_from(1.0_f64) - t195;
    (t191, t192, t194, t196)
}
