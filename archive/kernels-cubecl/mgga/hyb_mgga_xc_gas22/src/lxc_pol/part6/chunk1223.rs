//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1223/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1223<F: Float>(t191: F, t240: F, t6452: F, t8511: F, t8514: F, t2026: F, t6610: F, t3138: F, t8521: F, t763: F, t8512: F, t8518: F) -> (F, F, F, F, F, F) {
    let t23889 = t240 * t6452 * t191;
    let t23891 = t8511 * t23889 * t8514;
    let t23894 = t6610 * t2026 * t191;
    let t23896 = t3138 * t23894 * t8521;
    let t23905 = t8512 * t763;
    let t23909 = t8518 * t763;
    (t23889, t23891, t23894, t23896, t23905, t23909)
}
