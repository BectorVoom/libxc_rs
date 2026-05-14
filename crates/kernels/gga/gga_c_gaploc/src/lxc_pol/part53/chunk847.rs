//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 847/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk847<F: Float>(t13921: F, t7129: F, t2508: F, t2580: F, t47220: F, t13924: F, t7137: F, t47225: F, t1843: F, t39149: F, t7064: F, t12255: F, t2586: F, t39403: F, t948: F, t2541: F, t39022: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47711 = t7129 * t13921;
    let t47714 = t2508 * t2580 * t47220;
    let t47720 = t7137 * t13924;
    let t47723 = t2508 * t2580 * t47225;
    let t47731 = t7064 * t1843 * t39149;
    let t47734 = t2508 * t12255 * t2586;
    let t47737 = t7129 * t13924;
    let t47740 = t2508 * t39403 * t948;
    let t47749 = t2508 * t2541 * t39022;
    (t47711, t47714, t47720, t47723, t47731, t47734, t47737, t47740, t47749)
}
