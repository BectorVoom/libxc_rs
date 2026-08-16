//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1886/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886<F: Float>(t23270: F, t25038: F, t25039: F, t2553: F, t25040: F, t82074: F, t87712: F, t25193: F, t81591: F, t1484: F, t2249: F, t4119: F, t606: F) -> (F, F, F, F, F) {
    let t87924 = t25038 * t23270 * t25039 * t2553;
    let t87927 = t87712 * t82074 * t25040;
    let t87931 = t81591 * t25193;
    let t87953 = t2249 * t1484;
    let t87957 = t606 * t4119;
    (t87924, t87927, t87931, t87953, t87957)
}
