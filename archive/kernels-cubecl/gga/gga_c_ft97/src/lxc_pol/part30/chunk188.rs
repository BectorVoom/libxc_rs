//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 188/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk188<F: Float>(t1425: F, t263: F, t193: F, t1424: F, t265: F, t729: F, t734: F, t91: F, t26: F) -> (F, F, F, F, F) {
    let t1426 = t1425 * t263;
    let t1427 = t193 * t1426;
    let t1431 = t729 * t265 * t1424;
    let t1433 = t91 * t734;
    let t1434 = t1433 * t26;
    (t1426, t1427, t1431, t1433, t1434)
}
