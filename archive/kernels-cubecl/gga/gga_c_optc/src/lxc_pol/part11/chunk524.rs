//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 524/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk524<F: Float>(t1405: F, t2586: F, t940: F, t1397: F, t2367: F, t913: F, t1434: F, t999: F, t277: F, t95: F) -> (F, F, F, F, F) {
    let t3946 = t2586 * t1405;
    let t3947 = t940 * t3946;
    let t3951 = t2367 * t1397;
    let t3952 = t913 * t3951;
    let t3974 = t2367 * t1434;
    let t3975 = t999 * t3974;
    let t3980 = t95 * t277;
    (t3947, t3952, t3974, t3975, t3980)
}
