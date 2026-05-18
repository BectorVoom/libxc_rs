//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1309/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1309<F: Float>(t10350: F, t10364: F, t10373: F, t10394: F, t10408: F, t1270: F, t20530: F, t20545: F, t2104: F, t2112: F, t2116: F, t2133: F, t24354: F, t28549: F, t3231: F, t4046: F, t4051: F, t4068: F, t6355: F, t6363: F, t740: F, t8354: F, t8367: F, t8370: F, t8395: F, t8396: F) -> F {
    let t28623 = F::new(7.0) / F::new(2.0) * t4068 * t6355 + F::new(15.0) / F::new(4.0) * t10408 * t8396 - t8395 * t24354 - t10364 * t6355 / F::new(4.0) - t20545 * t4051 * t8396 / F::new(8.0) - F::new(6.0) * t6363 * t4051 * t2104 + F::new(4.0) * t2116 * t1270 * t8354 - t8367 * t10373 / F::new(2.0) - t3231 * t28549 - t8370 * t10373 / F::new(4.0) + F::new(4.0) * t2116 * t10350 * t740 + F::new(2.0) * t2116 * t4046 * t2104 - F::new(24.0) * t10394 * t8396 + F::new(24.0) * t20530 * t4051 * t2112 + F::new(7.0) / F::new(2.0) * t2133 * t10373 - F::new(6.0) * t6363 * t4046 * t2112;
    t28623
}
