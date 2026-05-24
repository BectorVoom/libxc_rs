//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 977/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk977<F: Float>(t14393: F, t1504: F, t14345: F, t14348: F, t14351: F, t14354: F, t14357: F, t14359: F, t14361: F, t14363: F, t14368: F, t14371: F, t14377: F, t14381: F, t14388: F, t14391: F) -> (F, F) {
    let t14394 = t1504 * t14393;
    let t14396 = t14345 / F::new(32.0) - t14348 / F::new(192.0) - F::new(19.0) / F::new(36.0) * t14351 + t14354 / F::new(4.0) - F::new(3.0) / F::new(128.0) * t14357 + t14359 / F::new(8.0) - t14361 / F::new(64.0) + t14363 - F::new(3.0) / F::new(8.0) * t14368 + F::new(11.0) / F::new(9.0) * t14371 + t14377 / F::new(864.0) - F::new(77.0) / F::new(27.0) * t14381 + F::new(209.0) / F::new(216.0) * t14388 - t14391 / F::new(8.0) + F::new(19.0) / F::new(48.0) * t14394;
    (t14394, t14396)
}
