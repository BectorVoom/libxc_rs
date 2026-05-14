//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 877/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk877<F: Float>(t322: F, t8936: F, t1115: F, t530: F, t1111: F, t24: F, t3097: F, t2586: F, t3147: F, t1133: F, t2855: F, t381: F) -> (F, F, F, F, F, F) {
    let t8937 = t322 * t8936;
    let t8940 = t530 * t1115;
    let t8941 = t1111 * t8940;
    let t8943 = t24 * t3097;
    let t8944 = t1111 * t8943;
    let t8946 = t2586 * t3147;
    let t8947 = t1133 * t8946;
    let t8950 = 1.0 / t381 / t2855;
    (t8937, t8941, t8944, t8946, t8947, t8950)
}
