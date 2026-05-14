//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1064/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1064<F: Float>(t20993: F, t20994: F, t20996: F, t20999: F, t21003: F, t21007: F, t21009: F, t21012: F, t21015: F, t21017: F, t21019: F, t21054: F, t21277: F, t21278: F, t21281: F, t21284: F, t21287: F, t21291: F, t21294: F, t21296: F, t21298: F, t21301: F, t21303: F, t21338: F) -> (F,) {
    let t21341 = t20993 + t20994 / 3.0 + t20996 / 128.0 + t20999 / 4.0 - t21003 / 256.0 + t21007 / 256.0 + 11.0 / 18.0 * t21009 + t21012 / 128.0 - t21015 / 128.0 + 2.0 / 9.0 * t21017 + t21019 / 256.0 + t21054 + t21277 - 19.0 / 144.0 * t21278 + t21281 / 12.0 - t21284 / 12.0 - 2.0 / 3.0 * t21287 - t21291 / 8.0 + t21294 / 4.0 - 2.0 / 9.0 * t21296 + t21298 / 24.0 + t21301 / 12.0 - t21303 / 12.0 + t21338;
    (t21341,)
}
