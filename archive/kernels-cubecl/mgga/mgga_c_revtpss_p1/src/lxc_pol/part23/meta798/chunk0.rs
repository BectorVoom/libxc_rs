//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2622/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622<F: Float>(t14370: F, t18259: F, t18562: F, t2626: F, t14330: F, t5819: F, t606: F, t749: F, t162: F, t50089: F, t2609: F, t5944: F) -> (F, F, F, F, F) {
    let t62274 = t18259 * t14370;
    let t62276 = t18562 * t2626;
    let t62282 = t14330 * t749 * t5819 * t606;
    let t62291 = t50089 * t162;
    let t62300 = t5944 * t2609;
    (t62274, t62276, t62282, t62291, t62300)
}
