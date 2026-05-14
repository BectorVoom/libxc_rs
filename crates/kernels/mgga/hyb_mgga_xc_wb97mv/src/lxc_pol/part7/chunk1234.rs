//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1234/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1234<F: Float>(t10622: F, t2025: F, t683: F, t10626: F, t2035: F, t6725: F, t10589: F, t1283: F, t8529: F, t8535: F, t136: F, t2003: F, t4072: F, t10869: F, t8195: F, t1234: F, t8626: F) -> (F, F, F, F, F, F, F, F) {
    let t30108 = t683 * t2025 * t10622;
    let t30111 = t2035 * t6725 * t10626;
    let t30114 = t683 * t2025 * t10589;
    let t30128 = t8529 * t1283;
    let t30132 = t8535 * t1283;
    let t30137 = t136 * t2003 * t4072;
    let t30139 = t8195 * t10869;
    let t30141 = t1234 * t8626;
    (t30108, t30111, t30114, t30128, t30132, t30137, t30139, t30141)
}
