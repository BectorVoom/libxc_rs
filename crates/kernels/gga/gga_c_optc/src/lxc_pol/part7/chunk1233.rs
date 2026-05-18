//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1233/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1233<F: Float>(t2606: F, t864: F, t14330: F, t7178: F, t25423: F, t3906: F, t19: F, t25425: F, t2662: F, t2264: F, t7982: F, t2670: F, t8384: F) -> (F, F, F, F, F, F, F) {
    let t25440 = t864 * t2606;
    let t25445 = t14330 * t7178;
    let t25453 = t3906 * t25423;
    let t25454 = t25425 * t19;
    let t25458 = t2662 * t25423;
    let t25468 = t7982 * t2264;
    let t25472 = t8384 * t2670;
    (t25440, t25445, t25453, t25454, t25458, t25468, t25472)
}
