//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1131/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1131<F: Float>(t7: F, t11186: F, t436: F, t1514: F, t3613: F, t1023: F, t4458: F, t3814: F, t7281: F, t2680: F, t3804: F, t1794: F, t224: F, t3619: F, t545: F, t9909: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t11187 = t11186 * t436;
    let t11188 = t3613 * t1514;
    let t11190 = t1023 * t4458;
    let t11192 = t7281 * t3814;
    let t11197 = t2680 * t3804;
    let t11203 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11192 * t545 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3619 * t1794 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11197 * t545 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t224 * t9909);
    (t11187, t11188, t11190, t11192, t11197, t11203)
}
