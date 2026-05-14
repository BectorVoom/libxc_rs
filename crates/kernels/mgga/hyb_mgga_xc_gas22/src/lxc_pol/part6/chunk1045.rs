//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1045/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1045<F: Float>(t10591: F, t10593: F, t10598: F, t10602: F, t10605: F, t10609: F, t10613: F, t6616: F, t6655: F, t8689: F, t8692: F, t8706: F, t10692: F) -> (F,) {
    let t10702 = 0.82524375e-1 * t10591 + 0.16504875e0 * t10593 - t6655 + 0.27595e0 * t6616 + 0.5519e0 * t8706 - t8689 - t8692 - 0.16557e0 * t10598 + 0.49671e0 * t10602 - 0.16557e0 * t10605 + 0.248355e0 * t10609 + 0.248355e0 * t10613;
    let t10703 = t10692 + t10702;
    (t10703,)
}
