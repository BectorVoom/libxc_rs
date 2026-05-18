//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 998/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk998<F: Float>(t193: F, t1949: F, t6654: F, t10050: F, t1924: F, t6668: F, t2268: F, t47: F, t1885: F) -> (F, F, F, F, F) {
    let t21998 = t193 * t6654 * t1949;
    let t22001 = t193 * t10050 * t1949;
    let t22004 = t193 * t1924 * t6668;
    let t22014 = F::new(1.0) / t47 / t2268;
    let t22015 = t1885 * t1885;
    (t21998, t22001, t22004, t22014, t22015)
}
