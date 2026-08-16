//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 665/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk665<F: Float>(t7: F, t3282: F, t675: F, t1318: F, t764: F, t26: F, t1794: F, t1329: F, t222: F, t568: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t3283 = t3282 * t675;
    let t3287 = t764 * t1318;
    let t3288 = t26 * t3287;
    let t3293 = F::cast_from(2.0_f64) * t1794;
    let t3294 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t3293);
    let t3300 = t222 * t568 * t1329;
    (t3283, t3287, t3288, t3293, t3294, t3300)
}
