//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 699/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk699<F: Float>(t130: F, t6922: F, t142: F, t127: F, t2067: F, t616: F, t2034: F, t3440: F, t6: F) -> (F, F, F, F, F, F) {
    let t6923 = t130 * t6922;
    let t6925 = 0.47892880429854730775e0 * t6923 * t142;
    let t6926 = t2067 * t127;
    let t6927 = t6926 * t616;
    let t6928 = t2034 * t6927;
    let t6931 = t3440 * t6;
    (t6923, t6925, t6926, t6927, t6928, t6931)
}
