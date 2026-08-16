//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 842/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk842<F: Float>(t22240: F, t4140: F, t10479: F, t21655: F, t4139: F, t1091: F, t19585: F, t2881: F, t15191: F, t5409: F, t15195: F, t5414: F) -> (F, F, F, F, F, F, F, F) {
    let t22368 = t4140 * t22240;
    let t22369 = t10479 * t22368;
    let t22372 = t4140 * t21655;
    let t22373 = t4139 * t22372;
    let t22376 = t19585 * t1091;
    let t22377 = t2881 * t22376;
    let t22380 = t15191 * t5409;
    let t22383 = t15195 * t5414;
    (t22368, t22369, t22372, t22373, t22376, t22377, t22380, t22383)
}
