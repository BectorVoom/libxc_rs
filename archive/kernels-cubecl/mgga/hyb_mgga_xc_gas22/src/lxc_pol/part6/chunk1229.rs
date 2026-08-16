//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1229/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1229<F: Float>(t39: F, t6288: F, t6291: F, t7942: F, t8309: F, t1890: F, t8326: F, t8322: F, t1248: F, t3023: F, t20290: F, t8304: F) -> (F, F, F, F, F, F, F) {
    let t24154 = t6288 * t39 * t6291;
    let t24158 = t7942 * t8309;
    let t24161 = t1890 * t8326;
    let t24163 = t7942 * t8322;
    let t24186 = t3023 * t1248;
    let t24205 = t20290 * t39;
    let t24216 = t1890 * t8304;
    (t24154, t24158, t24161, t24163, t24186, t24205, t24216)
}
