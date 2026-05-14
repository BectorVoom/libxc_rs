//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 966/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk966<F: Float>(t1476: F, t170: F, t1475: F, t1697: F, t475: F, t474: F, t16190: F, t49: F, t55: F, t204: F, t47: F, t5401: F, t4942: F, t500: F, t1489: F, t482: F) -> (F, F, F, F, F, F, F) {
    let t16204 = t1476 * t170;
    let t16205 = t1475 * t16204;
    let t16207 = t475 * t1697;
    let t16208 = t474 * t16207;
    let t16210 = t49 * t16190;
    let t16212 = f64::powf(t55, -0.25e1);
    let t16215 = t16212 * t47 * t5401 * t204;
    let t16217 = t4942 * t500;
    let t16219 = t1489 * t16204;
    let t16221 = t482 * t16207;
    (t16205, t16208, t16210, t16215, t16217, t16219, t16221)
}
