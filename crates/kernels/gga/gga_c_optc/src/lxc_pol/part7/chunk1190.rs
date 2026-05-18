//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1190/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1190<F: Float>(t2655: F, t7416: F, t2608: F, t2619: F, t874: F, t2658: F, t7421: F, t7907: F, t858: F, t7365: F, t224: F, t2263: F) -> (F, F, F, F, F, F) {
    let t24636 = t2655 * t7416;
    let t24644 = t874 * t2619 * t2608;
    let t24646 = t7421 * t2658;
    let t24652 = t7907 * t858;
    let t24654 = t2655 * t7365;
    let t24657 = F::new(1.0) / t224 / t2263;
    (t24636, t24644, t24646, t24652, t24654, t24657)
}
