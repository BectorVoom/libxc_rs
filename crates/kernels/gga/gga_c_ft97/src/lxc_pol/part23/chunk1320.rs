//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1320/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1320<F: Float>(t1212: F, t28719: F, t10479: F, t112790: F, t112795: F, t112803: F, t112821: F, t112831: F, t1508: F, t1901: F, t19404: F, t19409: F, t19418: F, t19465: F, t19815: F, t24908: F, t2862: F, t29207: F, t319: F, t4146: F, t446: F, t4965: F, t4973: F, t5225: F, t53797: F, t56819: F, t6353: F, t6386: F, t6393: F, t835: F, t840: F, t871: F, t98751: F, t99672: F) -> (F, F) {
    let t126018 = t28719 * t1212;
    let t126028 = 2.0 / 27.0 * t1901 * t10479 * t24908 * t4965 - t446 * t835 * t6393 * t4973 / 9.0 + 4.0 / 9.0 * t53797 * t99672 * t19815 + t112795 + t112803 - 2.0 / 3.0 * t446 * t2862 * t871 * t6386 * t5225 - 2.0 / 3.0 * t446 * t2862 * t6353 * t19404 + 4.0 / 3.0 * t446 * t2862 * t1508 * t19409 - 4.0 / 27.0 * t1901 * t56819 * t29207 * t19465 - t112821 + t112831 + 2.0 / 9.0 * t1901 * t112790 * t4146 + 4.0 / 3.0 * t446 * t2862 * t319 * t126018 + 2.0 / 3.0 * t446 * t840 * t6353 * t19418 - 4.0 / 27.0 * t98751;
    (t126018, t126028)
}
