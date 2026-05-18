//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 929/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk929<F: Float>(t178: F, t2104: F, t1270: F, t173: F, t180: F, t3227: F, t3232: F, t3245: F, t3246: F, t3252: F, t3255: F, t3258: F, t6355: F, t747: F, t751: F, t8354: F, t8373: F, t8395: F, t8396: F, t8399: F, t8402: F, t8410: F, t8415: F, t8418: F) -> (F, F) {
    let t8423 = t178 * t2104;
    let t8431 = F::new(15.0) / F::new(2.0) * t8395 * t8396 - F::new(4.0) * t8399 * t3246 - F::new(5.0) / F::new(2.0) * t8402 * t8396 - F::new(2.0) * t3245 * t6355 + t747 * t8354 * t180 / F::new(2.0) + t8410 * t3246 / F::new(2.0) + t3252 * t6355 / F::new(4.0) + t8415 * t8396 / F::new(8.0) - F::new(4.0) * t8418 * t1270 - F::new(8.0) * t3255 * t3227 - t8423 * t3232 - F::new(2.0) * t3258 * t8373 - F::new(4.0) * t751 * t8354 - t173 * t8354 * t180;
    (t8423, t8431)
}
