//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 902/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk902<F: Float>(t2104: F, t2111: F, t2112: F, t6359: F, t180: F, t3227: F, t1270: F, t2124: F, t6383: F, t2132: F, t6394: F, t181: F, t178: F, t173: F, t3232: F, t3245: F, t3246: F, t3252: F, t3255: F, t3258: F, t6355: F, t747: F, t751: F, t8354: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8367 = t2111 * t2104;
    let t8370 = t6359 * t2112;
    let t8373 = t180 * t3227;
    let t8395 = t2111 * t1270;
    let t8396 = t180 * t2112;
    let t8399 = t2124 * t3227;
    let t8402 = t6383 * t1270;
    let t8410 = t2132 * t3227;
    let t8415 = t6394 * t1270;
    let t8418 = t2104 * t181;
    let t8423 = t178 * t2104;
    let t8431 = 15.0 / 2.0 * t8395 * t8396 - 4.0 * t8399 * t3246 - 5.0 / 2.0 * t8402 * t8396 - 2.0 * t3245 * t6355 + t747 * t8354 * t180 / 2.0 + t8410 * t3246 / 2.0 + t3252 * t6355 / 4.0 + t8415 * t8396 / 8.0 - 4.0 * t8418 * t1270 - 8.0 * t3255 * t3227 - t8423 * t3232 - 2.0 * t3258 * t8373 - 4.0 * t751 * t8354 - t173 * t8354 * t180;
    (t8367, t8370, t8373, t8395, t8396, t8402, t8415, t8418, t8423, t8431)
}
