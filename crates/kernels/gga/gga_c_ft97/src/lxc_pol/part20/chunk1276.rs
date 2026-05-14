//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1276/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1276<F: Float>(t1882: F, t29166: F, t29101: F, t10703: F, t113222: F, t114222: F, t114271: F, t114488: F, t11593: F, t15299: F, t15308: F, t15312: F, t15313: F, t15345: F, t15522: F, t1901: F, t193: F, t24873: F, t24890: F, t25253: F, t25368: F, t28533: F, t2867: F, t2881: F, t2883: F, t28859: F, t295: F, t312: F, t3746: F, t4167: F, t4260: F, t446: F, t53797: F, t7045: F, t72190: F, t840: F, t89: F, t99628: F, t99672: F) -> (F,) {
    let t114499 = 2.0 / 27.0 * t1882 * t29166;
    let t114509 = 2.0 / 9.0 * t1882 * t29101;
    let t114529 = -2.0 / 9.0 * t1901 * t15312 * t24873 * t15522 + 4.0 / 9.0 * t53797 * t99672 * t15308 + 4.0 / 9.0 * t53797 * t114271 * t15313 + t89 * t193 * t295 * t114488 * t312 / 3.0 + 2.0 / 3.0 * t446 * t840 * t28859 * t2867 + t114499 - 2.0 / 9.0 * t99628 - 4.0 / 9.0 * t11593 * t2881 * t25368 * t3746 + 2.0 / 9.0 * t1901 * t24890 * t15345 + t114509 + 2.0 / 3.0 * t446 * t840 * t25253 * t4167 + 8.0 / 3.0 * t1901 * t72190 * t7045 * t2867 - 4.0 / 9.0 * t1901 * t15299 * t113222 - 2.0 / 9.0 * t1901 * t10703 * t28533 * t2883 - 4.0 / 9.0 * t1901 * t15312 * t114222 * t4260;
    (t114529,)
}
