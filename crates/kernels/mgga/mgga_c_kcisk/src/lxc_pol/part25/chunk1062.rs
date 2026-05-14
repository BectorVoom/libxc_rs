//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1062/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1062<F: Float>(t222: F, t227: F, t220: F, t3277: F, t1056: F, t213: F, t22192: F, t224: F, t3278: F, t3283: F, t5562: F, t5565: F, t967: F, t15783: F, t2063: F, t3289: F, t1060: F, t229: F, t3290: F, t3293: F, t5570: F, t5573: F, zeta_threshold: F) -> (F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t22195 = t3277 * t220;
    let t22206 = piecewise3(t223, 0.0, -8.0 / 27.0 * t22192 * t3278 + 16.0 / 9.0 * t22195 * t967 * t1056 + 4.0 / 9.0 * t5562 * t3283 + 8.0 / 3.0 * t224 * t967 - 8.0 * t5565 * t213);
    let t22207 = t15783 * t2063;
    let t22210 = t3289 * t220;
    let t22221 = piecewise3(t228, 0.0, -8.0 / 27.0 * t22207 * t3290 - 16.0 / 9.0 * t22210 * t967 * t1060 + 4.0 / 9.0 * t5570 * t3293 - 8.0 / 3.0 * t229 * t967 + 8.0 * t5573 * t213);
    (t22206, t22221)
}
