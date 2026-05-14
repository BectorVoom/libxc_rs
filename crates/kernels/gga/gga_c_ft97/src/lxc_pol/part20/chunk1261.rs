//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1261/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1261<F: Float>(t1882: F, t29274: F, t15128: F, t25213: F, t112405: F, t113051: F, t113309: F, t15157: F, t15162: F, t15290: F, t15369: F, t15425: F, t15460: F, t1901: F, t2405: F, t2413: F, t24869: F, t24898: F, t24936: F, t25271: F, t2749: F, t2862: F, t2881: F, t29082: F, t29245: F, t29399: F, t296: F, t4139: F, t4162: F, t446: F, t55937: F, t6360: F, t6393: F, t72443: F, t840: F) -> (F, F) {
    let t114001 = 2.0 / 27.0 * t1882 * t29274;
    let t114011 = t15128 * t25213;
    let t114037 = -2.0 / 3.0 * t1901 * t15369 * t6360 * t15425 + t1901 * t2881 * t29082 * t2413 / 9.0 + 2.0 / 27.0 * t1901 * t4139 * t29082 * t2405 + 4.0 / 3.0 * t446 * t2862 * t6393 * t4162 + t114001 + 4.0 / 27.0 * t1901 * t15290 * t113309 + 2.0 / 27.0 * t1901 * t15290 * t113051 + 4.0 / 3.0 * t446 * t296 * t112405 + 2.0 / 3.0 * t446 * t296 * t114011 + 2.0 / 3.0 * t446 * t840 * t2749 * t29245 + 2.0 / 27.0 * t1901 * t55937 * t24869 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t15157 - 4.0 / 9.0 * t1901 * t72443 * t24936 + 2.0 / 3.0 * t446 * t840 * t2749 * t29399 - 2.0 / 3.0 * t1901 * t15460 * t25271 * t15162;
    (t114011, t114037)
}
