//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1069/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1069<F: Float>(t2121: F, t21420: F, t337: F, t2134: F, t2074: F, t2122: F, t2147: F, t2120: F, t20270: F, t2276: F, t2281: F, t20695: F, t274: F, t20432: F, t6328: F, t8782: F) -> (F, F, F, F, F) {
    let t21422 = t2121 * t337 * t21420;
    let t21424 = t2134 * t21422 / 24.0;
    let t21427 = t2147 * t337 * t2122 * t2074;
    let t21429 = t2120 * t21427 / 8.0;
    let t21430 = t2276 * t20270;
    let t21431 = t21430 * t2281;
    let t21438 = t274 * t20695;
    let t21445 = t8782 * t20432 * t6328 / 16.0;
    (t21424, t21429, t21431, t21438, t21445)
}
