//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 268/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk268<F: Float>(t159: F, t794: F, t222: F, t228: F, t216: F) -> (F, F, F, F) {
    let t795 = t794 * t159;
    let t797 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t795 * t222;
    let t798 = t159 * t228;
    let t799 = t216 * t798;
    (t795, t797, t798, t799)
}
