//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1199/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1199<F: Float>(t21440: F, t21461: F, t21511: F, t21558: F, t21580: F, t21613: F, t21654: F, t21713: F, t21767: F, t21820: F, t21868: F, t21918: F, t21959: F, t22004: F, t22046: F, t22096: F, t41: F, t61: F) -> (F,) {
    let t22102 = t41 * t61 * (t21440 + t21461 + t21511 + t21558 + t21580 + t21613 + t21654 + t21713 + t21767 + t21820 + t21868 + t21918 + t21959 + t22004 + t22046 + t22096);
    (t22102,)
}
