//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1108/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1108<F: Float>(t2992: F, t5165: F, t3058: F, t5197: F, t2916: F, t2973: F, t5117: F, t2934: F, t5218: F, t8700: F, t5311: F, t8487: F) -> (F, F, F, F, F, F, F) {
    let t44583 = t5165 * t2992;
    let t44742 = t5197 * t3058;
    let t44909 = t5197 * t2916;
    let t44914 = t5117 * t2973;
    let t45045 = t5117 * t2934;
    let t45062 = t5218 * t8700;
    let t45304 = t8487 * t5311;
    (t44583, t44742, t44909, t44914, t45045, t45062, t45304)
}
