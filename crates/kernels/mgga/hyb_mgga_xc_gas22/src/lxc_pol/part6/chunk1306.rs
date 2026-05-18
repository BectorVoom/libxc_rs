//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1306/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1306<F: Float>(t143: F, t28311: F, t3227: F, t3246: F, t4046: F, t6394: F, t10350: F, t10411: F, t173: F, t178: F, t180: F, t181: F, t2111: F, t2124: F, t2132: F, t3255: F, t4068: F, t747: F, t751: F, t8354: F, t8395: F, t8396: F, t8402: F, t8415: F, t8418: F) -> (F, F, F, F) {
    let t145 = F::new(0.135e1) < t143;
    let t28459 = piecewise3::<f64>(t145, F::new(0.0), t28311);
    let t28476 = t3246 * t3227;
    let t28505 = t3227 * t3227;
    let t28530 = t6394 * t4046;
    let t28538 = F::new(30.0) * t8395 * t28476 - F::new(10.0) * t8402 * t28476 + t8415 * t28476 / F::new(2.0) - F::new(8.0) * t28505 * t181 + t747 * t28459 * t180 / F::new(2.0) - F::new(8.0) * t10411 * t8354 - F::new(4.0) * t8418 * t4046 - F::new(8.0) * t3255 * t10350 - F::new(4.0) * t751 * t28459 - t173 * t28459 * t180 - F::new(2.0) * t178 * t28505 * t180 - F::new(4.0) * t2124 * t28505 * t180 + t2132 * t28505 * t180 / F::new(2.0) + t28530 * t8396 / F::new(8.0) - F::new(75.0) / F::new(2.0) * t4068 * t8396 + F::new(15.0) / F::new(2.0) * t2111 * t4046 * t8396;
    (t28459, t28476, t28505, t28538)
}
