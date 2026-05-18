//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1141/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1141<F: Float>(t1160: F, t1304: F, t11346: F, t1535: F, t2857: F, t3742: F, t11267: F, t11279: F, t11283: F, t11315: F, t11320: F, t11329: F, t11336: F, t11343: F, t2821: F, t2824: F, t2834: F, t2838: F, t3665: F, t3680: F, t3688: F, t4491: F, t7643: F, t7800: F, t9639: F, t9663: F, t9670: F, t9678: F, t9700: F, sigma2: F) -> (F, F, F, F, F) {
    let t11347 = t1304 * t1160;
    let t11348 = t11347 * sigma2;
    let t11349 = t11346 * t11348;
    let t11353 = t1535 * t2857 * sigma2;
    let t11354 = t11353 * t3742;
    let t11357 = -F::new(16.0) / F::new(9.0) * t9700 * t4491 - F::new(8.0) / F::new(3.0) * t7643 * t11267 - F::new(40.0) / F::new(3.0) * t7800 * t11315 * t2824 - F::new(8.0) / F::new(3.0) * t2834 * t11320 + F::new(400.0) / F::new(9.0) * t3680 * t11279 + F::new(8.0) / F::new(3.0) * t2838 * t11283 + F::new(400.0) / F::new(9.0) * t3688 * t11279 - F::new(100.0) / F::new(9.0) * t11329 * t3665 - F::new(8.0) / F::new(9.0) * t2821 * t11320 + F::new(64.0) / F::new(27.0) * t9670 * t11336 + F::new(64.0) / F::new(9.0) * t9639 * t11336 - F::new(128.0) / F::new(81.0) * t9678 * t11343 + F::new(800.0) / F::new(81.0) * t9663 * t11349 - F::new(64.0) / F::new(27.0) * t9663 * t11354;
    (t11348, t11349, t11353, t11354, t11357)
}
