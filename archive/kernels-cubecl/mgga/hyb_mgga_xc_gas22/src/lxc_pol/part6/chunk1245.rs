//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1245/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1245<F: Float>(t2599: F, t260: F, t7109: F, t1409: F, t7075: F, t3513: F, t7150: F, t2559: F, t3524: F, t1414: F, t7058: F, t2576: F, t3557: F) -> (F, F, F, F, F, F, F) {
    let t25432 = t260 * t2599;
    let t25436 = t260 * t7109;
    let t25468 = t7075 * t1409;
    let t25520 = t3513 * t7150;
    let t25556 = t3524 * t2559;
    let t25561 = t1414 * t7058;
    let t25624 = t3557 * t2576;
    (t25432, t25436, t25468, t25520, t25556, t25561, t25624)
}
