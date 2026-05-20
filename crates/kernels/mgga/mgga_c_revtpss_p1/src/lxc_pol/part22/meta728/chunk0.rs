//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2784/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2784<F: Float>(t2394: F, t2475: F, t10069: F, t10929: F, t138: F, t785: F, t9302: F, t2786: F, t10073: F, t10920: F, t10871: F, t2645: F) -> (F, F, F, F, F, F) {
    let t40236 = t2475 * t2394;
    let t40267 = t10069 * t10929;
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40273 = t10073 * t10920;
    let t40284 = t10871 * t2645;
    (t40236, t40267, t40270, t40271, t40273, t40284)
}
