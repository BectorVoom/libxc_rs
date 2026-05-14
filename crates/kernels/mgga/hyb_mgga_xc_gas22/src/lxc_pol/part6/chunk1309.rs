//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1309/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1309<F: Float>(t2868: F, t30599: F, t1149: F, t1167: F, t1169: F, t22639: F, t22954: F, t26727: F, t26886: F, t2876: F, t30571: F, t30574: F, t30578: F, t30586: F, t30596: F, t30600: F, t30604: F, t4524: F, t518: F, t9636: F) -> (F, F) {
    let t30607 = t2868 * t30599;
    let t30610 = 3024.0 * t518 * t22639 * t4524 * t2876 + 256.0 / 9.0 * t22954 * t30571 + 12800.0 / 729.0 * t1169 * t30574 * t30578 + 12800.0 / 729.0 * t1167 * t30574 * t30578 + 6400.0 / 243.0 * t1167 * t26727 * t30586 + 6400.0 / 243.0 * t1169 * t26727 * t30586 + 6400.0 / 81.0 * t1149 * t26727 * t30586 - 3200.0 / 81.0 * t26886 * t30596 + 3200.0 / 81.0 * t30600 * t9636 - 1600.0 / 27.0 * t30604 * t9636 + 8000.0 / 27.0 * t30607 * t9636;
    (t30607, t30610)
}
