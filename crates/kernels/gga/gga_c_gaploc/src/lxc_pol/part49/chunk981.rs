//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 981/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk981<F: Float>(t1841: F, t47484: F, t7289: F, t2576: F, t39347: F, t43166: F, t43168: F, t43170: F, t47673: F, t47677: F, t47681: F, t47685: F, t47687: F, t47690: F, t13937: F, t731: F) -> (F, F) {
    let t47693 = t1841 * t7289 * t47484;
    let t47696 = t1841 * t39347 * t2576;
    let t47699 = -0.76905262301422242837e-2 * t47673 + 0.76905262301422242837e-2 * t47677 + 0.92286314761706691403e-1 * t47681 - 0.46143157380853345701e-1 * t47685 + 0.32043859292259267849e-3 * t47687 + 0.32043859292259267849e-3 * t47690 - 0.17090058289204942852e-2 * t47693 + 0.25635087433807414279e-2 * t47696 - t43166 - t43168 + 0.76905262301422242837e-2 * t43170;
    let t47702 = t731 * t13937;
    (t47699, t47702)
}
