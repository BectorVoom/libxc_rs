//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1482/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1482<F: Float>(t341: F, t42018: F, t42031: F, t12032: F, t342: F, t11902: F, t378: F, t3046: F, t3259: F, t3075: F, t11199: F, t988: F) -> (F, F, F, F, F, F) {
    let t42033 = (t42018 + t42031) * t341;
    let t42038 = t342 * t12032;
    let t42041 = t11902 * t378;
    let t42044 = t3046 * t3259;
    let t42047 = t3075 * t3075;
    let t42051 = t988 * t11199;
    (t42033, t42038, t42041, t42044, t42047, t42051)
}
