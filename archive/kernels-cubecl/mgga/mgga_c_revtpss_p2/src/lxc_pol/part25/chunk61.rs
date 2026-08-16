//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 61/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk61<F: Float>(t128: F, t131: F, t134: F, t141: F, t130: F) -> (F, F, F, F) {
    let t143 = F::cast_from(0.379785e1_f64) * t131 + F::cast_from(0.8969e0_f64) * t128 + F::cast_from(0.204775e0_f64) * t134 + F::cast_from(0.123235e0_f64) * t141;
    let t146 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t143;
    let t147 = F::ln(t146);
    let t149 = F::cast_from(0.621814e-1_f64) * t130 * t147;
    (t143, t146, t147, t149)
}
