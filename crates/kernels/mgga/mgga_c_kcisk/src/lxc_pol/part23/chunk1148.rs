//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1148/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1148<F: Float>(t3786: F, t6332: F, t32260: F, t1482: F, t1493: F, t394: F, t9498: F, t4214: F) -> (F, F, F, F, F, F) {
    let t32261 = t6332 * t3786;
    let t32262 = t32260 * t32261;
    let t32264 = t1482 * t1493;
    let t32266 = t1482 * t394;
    let t32267 = t32266 * t9498;
    let t32269 = t4214 * t394;
    (t32261, t32262, t32264, t32266, t32267, t32269)
}
