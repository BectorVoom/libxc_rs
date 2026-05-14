//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1073/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1073<F: Float>(t7: F, t132: F, t1023: F, t4458: F, t3814: F, t7281: F, t2680: F, t3804: F, t1794: F, t224: F, t3619: F, t545: F, t9909: F, t3925: F, t7292: F, t2688: F, t3938: F, t10325: F, t341: F, t3627: F, t675: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t11190 = t1023 * t4458;
    let t11192 = t7281 * t3814;
    let t11197 = t2680 * t3804;
    let t11203 = piecewise3(t8, 0.0, -8.0 / 27.0 * t11192 * t545 + 16.0 / 9.0 * t3619 * t1794 + 4.0 / 9.0 * t11197 * t545 + 4.0 / 3.0 * t224 * t9909);
    let t11204 = t7292 * t3925;
    let t11209 = t2688 * t3938;
    let t11215 = piecewise3(t133, 0.0, -8.0 / 27.0 * t11204 * t675 - 16.0 / 9.0 * t3627 * t1794 + 4.0 / 9.0 * t11209 * t675 + 4.0 / 3.0 * t341 * t10325);
    (t11190, t11192, t11197, t11203, t11204, t11209, t11215)
}
