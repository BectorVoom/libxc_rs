//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 633/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk633<F: Float>(t13149: F, t9438: F, t825: F, t313: F, t9014: F, t1645: F, t3251: F, t3009: F, t3209: F) -> (F, F, F, F, F, F) {
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13152 = 0.15976219147466979032e-1 * t13151;
    let t13153 = t313 * t9014;
    let t13154 = t1645 * t3251;
    let t13156 = 0.42900587942220512003e1 * t13153 * t13154;
    let t13157 = t3009 * t3209;
    (t13150, t13152, t13153, t13154, t13156, t13157)
}
