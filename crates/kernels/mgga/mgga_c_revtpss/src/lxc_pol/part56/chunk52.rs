//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 52/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk52<F: Float>(t128: F, t131: F, t134: F, t141: F, t130: F) -> (F, F, F, F) {
    let t143 = 0.379785e1 * t131 + 0.8969e0 * t128 + 0.204775e0 * t134 + 0.123235e0 * t141;
    let t146 = 1.0 + 0.16081979498692535067e2 / t143;
    let t147 = f64::ln(t146);
    let t149 = 0.621814e-1 * t130 * t147;
    (t143, t146, t147, t149)
}
