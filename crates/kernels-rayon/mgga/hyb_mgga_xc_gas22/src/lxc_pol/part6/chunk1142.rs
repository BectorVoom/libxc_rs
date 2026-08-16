//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1142/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1142(t1539: f64, t2850: f64, t3742: f64, t412: f64, t7580: f64, t3957: f64, t647: f64, t3664: f64, t2824: f64, t4576: f64, t1535: f64, t11343: f64, t11349: f64, t11354: f64, t3757: f64, t7811: f64, t9521: f64, t9533: f64, t9542: f64, t9575: f64, t9632: f64, t9639: f64, t9642: f64, t9654: f64, t9667: f64, t9670: f64, sigma0: f64, sigma2: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
    let t11399 = 128.0_f64 / 81.0_f64 * t9667 * t11361 + 800.0_f64 / 81.0_f64 * t9670 * t11349 - 128.0_f64 / 27.0_f64 * t9642 * t11343 + 800.0_f64 / 27.0_f64 * t9632 * t11349 - 64.0_f64 / 9.0_f64 * t9632 * t11354 + 128.0_f64 / 27.0_f64 * t9654 * t11361 + 800.0_f64 / 27.0_f64 * t9639 * t11349 - 64.0_f64 / 27.0_f64 * t3757 * t11379 + 700.0_f64 / 3.0_f64 * t9575 * t11383 + 32.0_f64 / 9.0_f64 * t7811 * t11386 + 200.0_f64 / 9.0_f64 * t9521 * t11383 - 200.0_f64 / 9.0_f64 * t9521 * t11392 + 100.0_f64 / 3.0_f64 * t9533 * t11383 - 500.0_f64 / 3.0_f64 * t9542 * t11392;
    (t11360, t11376, t11377, t11378, t11379, t11382, t11383, t11386, t11391, t11392, t11399)
}
