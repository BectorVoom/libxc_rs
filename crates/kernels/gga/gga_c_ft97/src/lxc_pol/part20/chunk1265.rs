//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1265/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1265<F: Float>(t25253: F, t4299: F, t29113: F, t8392: F, t1882: F, t29247: F, t7047: F, t8232: F, t29313: F, t1248: F, t98702: F, t6386: F, t668: F, t10683: F, t10703: F, t112657: F, t1255: F, t14889: F, t1508: F, t15133: F, t15201: F, t15369: F, t15433: F, t1901: F, t24898: F, t24949: F, t296: F, t4255: F, t446: F, t6365: F, t840: F, t99098: F, t99169: F, t99180: F) -> (F, F, F, F) {
    let t114182 = t25253 * t4299;
    let t114194 = 2.0 / 27.0 * t8392 * t29113;
    let t114196 = 2.0 / 9.0 * t1882 * t29247;
    let t114197 = t8232 * t7047;
    let t114211 = 4.0 / 9.0 * t1882 * t29313;
    let t114214 = t98702 * t1248;
    let t114222 = t6386 * t668;
    let t114227 = -2.0 / 3.0 * t446 * t296 * t114182 - t446 * t840 * t1508 * t14889 / 3.0 - 2.0 * t446 * t296 * t112657 - t114194 - t114196 + 4.0 / 27.0 * t114197 - 2.0 / 9.0 * t1901 * t99098 * t15201 - 2.0 / 3.0 * t1901 * t15369 * t24898 * t15433 - 2.0 * t446 * t10683 * t1255 * t24949 - t114211 - 4.0 / 9.0 * t99169 + t99180 / 9.0 - t446 * t296 * t114214 / 3.0 + 2.0 / 3.0 * t446 * t840 * t15133 * t6365 - 2.0 / 9.0 * t1901 * t10703 * t114222 * t4255;
    (t114182, t114214, t114222, t114227)
}
