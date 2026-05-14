//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 798/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk798<F: Float>(t8057: F, t894: F, t287: F, t297: F, t7835: F, t914: F, t2712: F, t2715: F, t7274: F, t916: F, t913: F, t2573: F, t909: F, t911: F, t2367: F, t2602: F) -> (F, F, F, F, F, F, F, F) {
    let t8058 = t894 * t8057;
    let t8062 = t287 * t7835 * t297;
    let t8063 = t914 * t8062;
    let t8066 = t2712 * t2715;
    let t8068 = t7274 * t916;
    let t8069 = t913 * t8068;
    let t8072 = t909 * t2573 * t911;
    let t8075 = t2367 * t2602;
    (t8058, t8062, t8063, t8066, t8068, t8069, t8072, t8075)
}
