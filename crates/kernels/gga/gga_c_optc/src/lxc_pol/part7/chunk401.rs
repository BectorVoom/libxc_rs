//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 401/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk401<F: Float>(t1923: F, t1926: F, t1928: F, t193: F, t1949: F, t195: F, t197: F, t750: F, t201: F, t5: F, t743: F) -> (F, F, F) {
    let t1953 = -t1923 + F::new(400.0) / F::new(27.0) * t1926 - F::new(25.0) / F::new(9.0) * t193 * t195 * t1928 * t197 - F::new(25.0) / F::new(9.0) * t193 * t750 * t1949;
    let t1955 = t5 * t1953 * t201;
    let t1956 = t743 * t1955;
    (t1953, t1955, t1956)
}
