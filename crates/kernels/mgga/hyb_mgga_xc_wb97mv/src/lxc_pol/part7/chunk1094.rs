//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1094/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1094<F: Float>(t132: F, t2799: F, t3988: F, t10621: F, t11598: F, t1875: F, t339: F, t3649: F, t674: F, t11597: F, t259: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t11603 = t2799 * t3988;
    let t11609 = piecewise3(t133, 0.0, -8.0 / 27.0 * t11598 * t674 - 16.0 / 9.0 * t3649 * t1875 + 4.0 / 9.0 * t11603 * t674 + 4.0 / 3.0 * t339 * t10621);
    let t11611 = (t11597 + t11609) * t259;
    (t11603, t11611)
}
