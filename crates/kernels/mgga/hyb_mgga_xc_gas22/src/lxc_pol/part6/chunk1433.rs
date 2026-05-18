//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1433/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1433<F: Float>(t1539: F, t7580: F, t9503: F, t7573: F, t1535: F, t11353: F, t11335: F, t26226: F, t26231: F, t26846: F, t26850: F, t9632: F, t9639: F, t9642: F, t9654: F, t9663: F, t9667: F, t9670: F, t9678: F, sigma2: F) -> F {
    let t31069 = t1539 * t7580 * sigma2 * t9503;
    let t31074 = t1539 * t7573 * sigma2 * t9503;
    let t31083 = t1535 * t7580 * sigma2 * t9503;
    let t31086 = t11353 * t9503;
    let t31091 = t1535 * t7573 * sigma2 * t9503;
    let t31096 = t11335 * t9503;
    let t31105 = F::new(256.0) / F::new(9.0) * t9639 * t31069 + F::new(512.0) / F::new(81.0) * t9667 * t31074 + F::new(256.0) / F::new(27.0) * t9670 * t31069 + F::new(512.0) / F::new(27.0) * t9654 * t31074 - F::new(256.0) / F::new(27.0) * t9663 * t31083 - F::new(2048.0) / F::new(729.0) * t26226 * t31086 - F::new(512.0) / F::new(81.0) * t9678 * t31091 - F::new(512.0) / F::new(27.0) * t9642 * t31091 + F::new(2048.0) / F::new(729.0) * t26231 * t31096 - F::new(256.0) / F::new(9.0) * t9632 * t31083 + F::new(2048.0) / F::new(243.0) * t26850 * t31096 - F::new(2048.0) / F::new(243.0) * t26846 * t31086;
    t31105
}
