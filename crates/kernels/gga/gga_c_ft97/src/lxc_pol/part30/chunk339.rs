//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 339/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk339<F: Float>(t263: F, t6062: F, t193: F, t1425: F, t771: F, t1431: F, t1882: F, t1424: F, t258: F) -> (F, F, F, F, F, F) {
    let t6063 = t6062 * t263;
    let t6064 = t193 * t6063;
    let t6067 = t1425 * t771;
    let t6068 = t193 * t6067;
    let t6073 = t1882 * t1431 / 9.0;
    let t6074 = t258 * t1424;
    (t6063, t6064, t6067, t6068, t6073, t6074)
}
