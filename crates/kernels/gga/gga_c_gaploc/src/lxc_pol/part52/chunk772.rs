//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 772/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk772<F: Float>(t2293: F, t587: F, t9438: F, t9439: F, t12449: F, t7014: F, t2487: F, t9448: F, t12448: F, t2464: F, t4167: F, t883: F) -> (F, F, F, F, F) {
    let t40013 = t587 * t9438 * t9439 * t2293;
    let t40015 = t7014 * t12449;
    let t40019 = t2487 * t9438 * t9448 * t2293;
    let t40076 = t2487 * t2464 * t12448;
    let t40165 = t883 * t4167;
    (t40013, t40015, t40019, t40076, t40165)
}
