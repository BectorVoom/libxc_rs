//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 876/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk876<F: Float>(t14: F, t237: F, t6610: F, t7337: F, t7340: F, t7343: F, t7346: F, t7350: F, t7352: F, t7354: F, t1081: F, t1080: F, t2773: F) -> (F, F, F, F) {
    let t7357 = t237 * t14 * t6610;
    let t7359 = -F::new(0.47063e1) * t7337 + F::cast_from(0.31375333333333333334e1_f64) * t7340 - F::cast_from(0.36604555555555555556e1_f64) * t7343 - F::cast_from(0.16068111111111111111e1_f64) * t7346 + F::cast_from(0.28051666666666666666e0_f64) * t7350 - F::cast_from(0.56103333333333333332e0_f64) * t7352 - F::cast_from(0.6545388888888888889e0_f64) * t7354 - F::cast_from(0.46308888888888888888e0_f64) * t7357;
    let t7360 = t7359 * t1081;
    let t7363 = t2773 * t1080;
    (t7357, t7359, t7360, t7363)
}
