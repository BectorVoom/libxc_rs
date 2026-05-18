//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 306/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk306<F: Float>(t345: F, t13: F, t30: F, t1188: F) -> F {
    let t1207 = t345 * t345;
    let t1208 = F::new(1.0) / t1207;
    let t1209 = t13 * t1208;
    let t1210 = t30 * t30;
    let t1211 = F::new(1.0) / t1210;
    let t1212 = t1188 * t1211;
    let t1214 = F::new(0.16081824322151104822e2) * t1209 * t1212;
    t1214
}
