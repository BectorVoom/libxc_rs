//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 223/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk223<F: Float>(t136: F, t658: F, t166: F, t145: F, t108: F, t157: F, t110: F, t146: F) -> (F, F, F, F, F, F) {
    let t659 = t136 * t658;
    let t668 = t166 * t166;
    let t669 = F::cast_from(1.0_f64) / t668;
    let t670 = t145 * t669;
    let t671 = t157 * t108;
    let t673 = t146 * t671 * t110;
    (t659, t668, t669, t670, t671, t673)
}
