//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 960/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk960<F: Float>(t1655: F, t71: F, t2264: F, t341: F, t17: F, t8946: F, t8947: F, t120: F, t1570: F, t1559: F, t16: F, t2252: F) -> (F, F, F, F, F, F) {
    let t39918 = t71 * t1655;
    let t39922 = t341 * t2264;
    let t39926 = t8946 * t8947 * t17;
    let t39931 = t120 * t1570;
    let t39932 = t39931 * t1559;
    let t39942 = t8946 * t16;
    let t39976 = t341 * t2252;
    (t39918, t39922, t39926, t39932, t39942, t39976)
}
