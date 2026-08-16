//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1142/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1142<F: Float>(t1539: F, t2850: F, t3742: F, t412: F, t7580: F, t3957: F, t647: F, t3664: F, t2824: F, t4576: F, t1535: F, t11343: F, t11349: F, t11354: F, t3757: F, t7811: F, t9521: F, t9533: F, t9542: F, t9575: F, t9632: F, t9639: F, t9642: F, t9654: F, t9667: F, t9670: F, sigma0: F, sigma2: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11360 = t1539 * t2850 * sigma2;
    let t11361 = t11360 * t3742;
    let t11376 = t7580 * t412;
    let t11377 = t3957 * sigma0;
    let t11378 = t11377 * t647;
    let t11379 = t11376 * t11378;
    let t11382 = t1539 * tau0;
    let t11383 = t11382 * t3664;
    let t11386 = t4576 * t2824;
    let t11391 = t1535 * tau0;
    let t11392 = t11391 * t3664;
    let t11399 = F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t9667 * t11361 + F::cast_from(800.0_f64) / F::cast_from(81.0_f64) * t9670 * t11349 - F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t9642 * t11343 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t9632 * t11349 - F::cast_from(64.0_f64) / F::cast_from(9.0_f64) * t9632 * t11354 + F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t9654 * t11361 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t9639 * t11349 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3757 * t11379 + F::cast_from(700.0_f64) / F::cast_from(3.0_f64) * t9575 * t11383 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t11386 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t9521 * t11383 - F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t9521 * t11392 + F::cast_from(100.0_f64) / F::cast_from(3.0_f64) * t9533 * t11383 - F::cast_from(500.0_f64) / F::cast_from(3.0_f64) * t9542 * t11392;
    (t11360, t11376, t11377, t11378, t11379, t11382, t11383, t11386, t11391, t11392, t11399)
}
