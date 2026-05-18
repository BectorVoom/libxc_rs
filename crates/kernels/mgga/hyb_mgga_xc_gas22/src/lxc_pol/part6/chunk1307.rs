//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1307/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1307<F: Float>(t10350: F, t180: F, t2132: F, t10364: F, t10373: F, t10394: F, t10397: F, t10403: F, t10408: F, t10414: F, t20467: F, t20475: F, t2124: F, t24354: F, t3245: F, t3246: F, t3252: F, t3258: F, t4046: F, t4051: F, t4052: F, t6355: F, t6383: F, t8396: F, t8423: F) -> (F, F) {
    let t28549 = t180 * t10350;
    let t28571 = t2132 * t10350;
    let t28576 = t3252 * t24354 / F::new(2.0) + t10408 * t6355 / F::new(8.0) + t20475 * t4051 * t8396 / F::new(16.0) - F::new(2.0) * t10414 * t24354 - t8423 * t10373 - F::new(2.0) * t3258 * t28549 + F::new(15.0) / F::new(2.0) * t4052 * t6355 + F::new(85.0) / F::new(4.0) * t10364 * t8396 - F::new(4.0) * t3245 * t24354 - F::new(5.0) / F::new(2.0) * t10394 * t6355 - F::new(19.0) / F::new(8.0) * t20467 * t4051 * t8396 - F::new(4.0) * t2124 * t10350 * t3246 - F::new(2.0) * t10397 * t6355 - F::new(5.0) / F::new(2.0) * t6383 * t4046 * t8396 + t28571 * t3246 / F::new(2.0) + t10403 * t6355 / F::new(4.0);
    (t28549, t28576)
}
