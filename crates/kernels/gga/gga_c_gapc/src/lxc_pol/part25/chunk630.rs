//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 630/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk630<F: Float>(t1903: F, t1908: F, t198: F, t681: F, t137: F, t567: F) -> (F, F, F, F) {
    let t5211 = t1903 * M_PI;
    let t5214 = t198 * t1908;
    let t5215 = t5214 * t681;
    let t5216 = t567 * t137;
    (t5211, t5214, t5215, t5216)
}
