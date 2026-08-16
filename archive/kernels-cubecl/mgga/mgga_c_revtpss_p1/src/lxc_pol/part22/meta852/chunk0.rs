//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2993/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993<F: Float>(t4186: F, t4401: F, t606: F, t749: F, t14362: F, t9575: F, t123: F, t2630: F, t4392: F, t4398: F, t9318: F, t15071: F, t892: F) -> (F, F, F, F, F) {
    let t49911 = t4401 * t749 * t4186 * t606;
    let t49926 = t14362 * t9575;
    let t49929 = t4392 * t123 * t2630;
    let t49940 = t4398 * t9318;
    let t49950 = t15071 * t892;
    (t49911, t49926, t49929, t49940, t49950)
}
