//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 435/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk435<F: Float>(t2144: F, t688: F, t116: F, t1928: F, t2010: F, t1948: F, t627: F, t156: F) -> (F, F, F, F, F) {
    let t2145 = t2144 * t688;
    let t2148 = t2010 * t116 * t1928;
    let t2152 = t627 * t116 * t1948;
    let t2155 = t156 * t156;
    let t2156 = F::new(1.0) / t2155;
    (t2145, t2148, t2152, t2155, t2156)
}
