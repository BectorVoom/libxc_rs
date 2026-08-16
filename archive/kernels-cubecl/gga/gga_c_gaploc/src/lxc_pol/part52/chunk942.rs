//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 942/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk942<F: Float>(t12252: F, t2628: F, t13892: F, t5676: F, t12161: F, t2033: F, t2365: F, t2610: F, t13848: F, t7416: F, t12255: F, t769: F) -> (F, F, F, F, F) {
    let t47450 = t12252 * t2628;
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    let t47494 = t7416 * t13848;
    let t47496 = t769 * t12255;
    (t47450, t47488, t47492, t47494, t47496)
}
