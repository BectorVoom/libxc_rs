//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 684/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk684<F: Float>(t12399: F, t467: F, t12445: F, t1407: F, t2293: F, t587: F, t9438: F, t9439: F, t12449: F, t7014: F, t2487: F, t9448: F, t12448: F, t2464: F, t4167: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t39650 = t12399 * t467;
    let t40009 = t1407 * t12445;
    let t40013 = t587 * t9438 * t9439 * t2293;
    let t40015 = t7014 * t12449;
    let t40019 = t2487 * t9438 * t9448 * t2293;
    let t40076 = t2487 * t2464 * t12448;
    let t40165 = t883 * t4167;
    (t39650, t40009, t40013, t40015, t40019, t40076, t40165)
}
