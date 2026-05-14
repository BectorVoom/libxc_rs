//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 966/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk966<F: Float>(t7: F, t7274: F, t1179: F, t7281: F, t2680: F, t3: F, t1793: F, t1796: F, t1808: F, t224: F, t3619: F, t3622: F, t461: F, t8636: F, t1238: F, t7292: F, t2688: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9338 = 32.0 * t7274;
    let t9340 = t7281 * t1179;
    let t9343 = t2680 * t3;
    let t9353 = piecewise3(t8, 0.0, -8.0 / 27.0 * t9340 * t1808 + 16.0 / 9.0 * t9343 * t8636 + 4.0 / 9.0 * t3619 * t1796 + 8.0 / 3.0 * t224 * t1793 - 8.0 * t3622 * t461);
    let t9354 = t7292 * t1238;
    let t9357 = t2688 * t3;
    (t9338, t9340, t9353, t9354, t9357)
}
