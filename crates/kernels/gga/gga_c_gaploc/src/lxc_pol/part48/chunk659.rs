//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 659/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk659<F: Float>(t7892: F, t9439: F, t9448: F, t4348: F, t997: F, t1033: F, t5558: F, t1381: F, t2796: F, t22537: F, t822: F, t2012: F, t9804: F, t22542: F, t2021: F, t6109: F, t899: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27003 = t9439 * t7892;
    let t27007 = t9448 * t7892;
    let t27214 = t997 * t4348;
    let t27229 = t1033 * t5558;
    let t27232 = t2796 * t1381;
    let t28069 = t822 * t22537;
    let t28073 = t2012 * t9804;
    let t28309 = t822 * t22542;
    let t28412 = t2021 * t6109 * t899;
    (t27003, t27007, t27214, t27229, t27232, t28069, t28073, t28309, t28412)
}
