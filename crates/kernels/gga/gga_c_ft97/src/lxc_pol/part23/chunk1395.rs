//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1395/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1395<F: Float>(t1882: F, t31770: F, t10703: F, t114595: F, t114606: F, t114616: F, t114626: F, t114648: F, t11593: F, t127728: F, t1901: F, t19470: F, t19474: F, t24886: F, t29137: F, t29141: F, t31551: F, t31852: F, t31862: F, t319: F, t446: F, t684: F, t69996: F, t72397: F, t840: F, t882: F, t99665: F) -> (F,) {
    let t128099 = t1882 * t31770;
    let t128101 = 4.0 / 81.0 * t99665 + t114595 - t114606 - t1901 * t10703 * t31862 * t684 / 9.0 - t1901 * t10703 * t31852 * t684 / 9.0 - t114616 - t114626 - t446 * t840 * t319 * t127728 / 3.0 - t446 * t840 * t882 * t31551 / 3.0 - 4.0 / 3.0 * t1901 * t69996 * t29137 - 4.0 / 3.0 * t1901 * t72397 * t29141 - 2.0 / 3.0 * t1901 * t24886 * t19470 - 8.0 / 9.0 * t11593 * t24886 * t19474 + t128099 / 27.0 - t114648;
    (t128101,)
}
