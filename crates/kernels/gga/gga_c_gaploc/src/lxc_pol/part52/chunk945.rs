//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 945/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk945<F: Float>(t13934: F, t731: F, t13937: F, t2549: F, t12176: F, t2558: F, t943: F, t1843: F, t39149: F, t7064: F, t2562: F, t38974: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t47652 = t731 * t13934;
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    let t47702 = t731 * t13937;
    let t47731 = t7064 * t1843 * t39149;
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    (t47652, t47687, t47690, t47702, t47731, t47768, t47772)
}
