//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1180/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1180<F: Float>(t32398: F, t32427: F, t32479: F, t32510: F, t2748: F, t4536: F, t15087: F, t15094: F, t1611: F, t32225: F, t32228: F, t32235: F, t32240: F, t32243: F, t32246: F, t32336: F, t4565: F, t555: F, t9557: F, t9560: F) -> (F, F, F) {
    let t32512 = t32398 + t32427 + t32479 + t32510;
    let t32517 = t2748 * t4536;
    let t32520 = 4.0 * t15087 * t9560 - 6.0 * t15094 * t32517 - t1611 * t32336 + t32512 * t555 - t4565 * t9557 - t32225 + t32228 - t32235 + t32240 - t32243 - t32246;
    (t32512, t32517, t32520)
}
