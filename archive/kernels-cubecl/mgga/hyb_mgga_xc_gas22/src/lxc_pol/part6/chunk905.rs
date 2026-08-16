//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 905/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk905<F: Float>(t1230: F, t668: F, t545: F, t1796: F, t2997: F, t1189: F, t6012: F, t1890: F, t3011: F, t3017: F, t25: F, t460: F) -> (F, F, F, F, F, F, F) {
    let t7920 = t1230 * t668;
    let t7921 = t7920 * t545;
    let t7925 = t2997 * t1796;
    let t7933 = t6012 * t1189;
    let t7936 = F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t1890 * t3011;
    let t7938 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t1890 * t3017;
    let t7940 = F::cast_from(1.0_f64) / t25 / t460;
    (t7920, t7921, t7925, t7933, t7936, t7938, t7940)
}
