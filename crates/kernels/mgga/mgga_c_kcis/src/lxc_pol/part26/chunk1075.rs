//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1075/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1075<F: Float>(t1380: F, t6284: F, t6281: F, t1650: F, t5732: F, t22636: F, t12234: F, t7091: F, t1363: F, t21453: F, t1494: F, t6927: F, t4134: F, t7202: F, t3960: F, t7028: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t59401 = t6284 * t1380;
    let t59414 = t6281 * t1380;
    let t59578 = t1650 * t5732;
    let t59975 = t22636 * sigma2;
    let t60029 = t7091 * t12234;
    let t60299 = t21453 * t1363;
    let t60756 = t1494 * t6927;
    let t60761 = t4134 * t7202;
    let t60780 = t7028 * t3960;
    (t59401, t59414, t59578, t59975, t60029, t60299, t60756, t60761, t60780)
}
