//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2149/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2149<F: Float>(t25250: F, t87202: F, t87712: F, t25316: F, t82038: F, t1888: F, t232: F, t47439: F, t6646: F, t23110: F, t23185: F, t25272: F) -> (F, F, F, F) {
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    let t87726 = t1888 * t6646 * t47439 * t232;
    let t87729 = t23185 * t23110 * t25272;
    (t87714, t87718, t87726, t87729)
}
