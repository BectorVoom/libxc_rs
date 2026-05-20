//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3087/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087<F: Float>(t20365: F, t5079: F, t16862: F, t6449: F, t20337: F, t5087: F, t1134: F, t24312: F, t3407: F, t141: F, t3417: F, t81177: F) -> (F, F, F, F, F) {
    let t81523 = t20365 * t5079;
    let t81525 = t16862 * t6449;
    let t81527 = t5087 * t20337;
    let t81530 = t3407 * t24312 * t1134;
    let t81533 = t141 * t3417 * t81177;
    (t81523, t81525, t81527, t81530, t81533)
}
