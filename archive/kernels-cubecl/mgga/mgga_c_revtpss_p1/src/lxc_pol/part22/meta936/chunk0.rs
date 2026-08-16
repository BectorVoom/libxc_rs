//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3169/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3169<F: Float>(t1256: F, t17333: F, t12268: F, t29054: F, t12898: F, t1786: F, t17202: F, t372: F, t17708: F, t45769: F, t44546: F, t5340: F, t5342: F) -> (F, F, F, F, F, F) {
    let t57604 = t17333 * t1256;
    let t57606 = t29054 * t12268;
    let t57615 = t1786 * t12898;
    let t57621 = t372 * t17202;
    let t57631 = t45769 * t17708;
    let t57635 = t5340 * t44546 * t5342;
    (t57604, t57606, t57615, t57621, t57631, t57635)
}
