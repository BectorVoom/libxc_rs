//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1854/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1854<F: Float>(t1512: F, t81807: F, t25146: F, t2686: F, t81824: F, t81821: F, t23053: F, t4236: F, t13173: F, t6614: F, t23041: F, t13186: F, t6621: F) -> (F, F, F, F, F, F, F, F) {
    let t87243 = t81807 * t1512;
    let t87245 = t25146 * t2686;
    let t87247 = t81824 * t1512;
    let t87249 = t81821 * t1512;
    let t87251 = t23053 * t4236;
    let t87253 = t6614 * t13173;
    let t87255 = t23041 * t4236;
    let t87257 = t6621 * t13186;
    (t87243, t87245, t87247, t87249, t87251, t87253, t87255, t87257)
}
