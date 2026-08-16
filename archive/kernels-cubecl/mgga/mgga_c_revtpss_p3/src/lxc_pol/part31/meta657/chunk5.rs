//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2219/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2219<F: Float>(t1907: F, t5591: F, t25082: F, t8717: F, t29495: F, t7235: F, t5778: F, t28196: F, t28197: F, t28184: F, t7898: F, t5920: F, t648: F) -> (F, F, F, F, F) {
    let t108682 = t5591 * t1907;
    let t108685 = F::cast_from(6.0_f64) * t25082 * t8717 * t108682;
    let t108687 = F::cast_from(3.0_f64) * t7235 * t29495;
    let t108688 = t1907 * t5778;
    let t108691 = F::cast_from(4.0_f64) * t28196 * t28197 * t108688;
    let t108693 = F::cast_from(6.0_f64) * t7898 * t28184;
    let t108710 = t648 * t5920;
    (t108685, t108687, t108691, t108693, t108710)
}
