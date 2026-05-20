//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2400/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2400<F: Float>(t40861: F, t802: F, t10899: F, t794: F, t10902: F, t159: F, t216: F, t2475: F, t2645: F, t860: F, t231: F, t2782: F, t2783: F, t39714: F) -> (F, F, F, F, F, F) {
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40865 = t40864 * t10902;
    let t40868 = t216 * t159 * t2475;
    let t40888 = t860 * t2645;
    let t40894 = t2782 * t2783 * t39714 * t231;
    (t40862, t40864, t40865, t40868, t40888, t40894)
}
