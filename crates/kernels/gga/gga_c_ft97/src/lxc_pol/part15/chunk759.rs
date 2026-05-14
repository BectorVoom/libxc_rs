//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 759/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk759<F: Float>(t21655: F, t4140: F, t4139: F, t1091: F, t19585: F, t2881: F, t15191: F, t5409: F, t15195: F, t5414: F, t1212: F, t4635: F, t2875: F, t2874: F, t1248: F, t2882: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22372 = t4140 * t21655;
    let t22373 = t4139 * t22372;
    let t22376 = t19585 * t1091;
    let t22377 = t2881 * t22376;
    let t22380 = t15191 * t5409;
    let t22383 = t15195 * t5414;
    let t22386 = t4635 * t1212;
    let t22387 = t2875 * t22386;
    let t22388 = t2874 * t22387;
    let t22391 = t4635 * t1248;
    let t22392 = t2882 * t22391;
    (t22372, t22373, t22376, t22377, t22380, t22383, t22386, t22387, t22388, t22391, t22392)
}
