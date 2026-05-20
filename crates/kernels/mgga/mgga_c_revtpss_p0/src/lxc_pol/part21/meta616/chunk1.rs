//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2369/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2369<F: Float>(t138: F, t785: F, t9302: F, t2786: F, t10073: F, t10920: F, t231: F, t2760: F, t2782: F, t2783: F, t836: F, t10871: F, t14545: F, t39709: F) -> (F, F, F, F, F) {
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40273 = t10073 * t10920;
    let t40278 = t2782 * t2783 * t2760 * t836 * t231;
    let t40282 = t2782 * t14545 * t39709 * t10871;
    (t40270, t40271, t40273, t40278, t40282)
}
