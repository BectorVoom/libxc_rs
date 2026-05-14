//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1384/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1384<F: Float>(t697: F, t9904: F, t21969: F, t21972: F, t22464: F, t22467: F, t22468: F, t22472: F, t22473: F, t22478: F, t22481: F, t22484: F, t22487: F, t26831: F, t10266: F, t584: F, t591: F) -> (F, F) {
    let t33691 = t9904 * t697;
    let t33693 = t22464 - t22467 + 0.40020429009866666664e-2 * t22468 + t22472 - 0.43374325201206959369e-1 * t22473 - t22478 - t22481 + t22484 - t22487 + t26831 - t21969 - t21972 + 0.65061487801810439052e-1 * t33691;
    let t33697 = t584 * t10266 * t591;
    (t33693, t33697)
}
