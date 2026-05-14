//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 878/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk878<F: Float>(t3006: F, t898: F, t2995: F, t896: F, t3012: F, t3: F, t74: F, t83: F, t213: F, t12476: F, t2957: F, t12485: F, t866: F, t68: F, t71: F, t2966: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15279 = t898 * t3006;
    let t15283 = t2995 * t896;
    let t15285 = t3012 * t15283 * t898;
    let t15291 = 1.0 / t74 / t83 * t3 / 4.0;
    let t15292 = t15291 * t213;
    let t15294 = t2957 * t12476;
    let t15296 = t866 * t12485;
    let t15298 = t68 * t12485;
    let t15300 = 1.0/pow_3_2(t71);
    let t15301 = t15300 * t3;
    let t15302 = t15301 * t213;
    let t15304 = t2966 * t12476;
    (t15279, t15283, t15285, t15292, t15294, t15296, t15298, t15302, t15304)
}
