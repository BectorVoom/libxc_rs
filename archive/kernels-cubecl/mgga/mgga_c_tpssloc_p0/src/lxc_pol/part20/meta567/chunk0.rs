//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2126/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2126<F: Float>(t1030: F, t10477: F, t10472: F, t10475: F, t3128: F, t10903: F, t10948: F, t10890: F, t10898: F, t3103: F, t10904: F, t11002: F) -> (F, F, F, F, F, F, F) {
    let t42559 = t1030 * t10477;
    let t42561 = t10472 * t10475 * t42559;
    let t42565 = t10472 * t3128 * t42559;
    let t42570 = t10948 * t10903;
    let t42573 = t10948 * t10890;
    let t42578 = t10898 * t3103;
    let t42582 = t10904 * t11002;
    (t42559, t42561, t42565, t42570, t42573, t42578, t42582)
}
