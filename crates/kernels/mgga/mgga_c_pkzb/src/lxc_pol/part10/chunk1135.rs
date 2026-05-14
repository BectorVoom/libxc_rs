//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1135/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1135<F: Float>(t16190: F, t49: F, t55: F, t204: F, t47: F, t5401: F, t4942: F, t500: F, t1489: F, t16204: F, t16207: F, t482: F, t170: F, t50: F, t65: F, t16200: F, t16202: F, t16205: F, t16208: F) -> (F, F, F, F, F, F, F) {
    let t16210 = t49 * t16190;
    let t16212 = f64::powf(t55, -0.25e1);
    let t16215 = t16212 * t47 * t5401 * t204;
    let t16217 = t4942 * t500;
    let t16219 = t1489 * t16204;
    let t16221 = t482 * t16207;
    let t16224 = t65 * t50 * t170;
    let t16226 = -0.28769444444444444444e1 * t16200 + 0.27618666666666666667e2 * t16202 - 0.10229135802469135803e2 * t16205 + 0.89504938271604938273e1 * t16208 + 0.31310740740740740741e1 * t16210 + 0.366775e-1 * t16215 - 0.58684e0 * t16217 + 0.65204444444444444445e0 * t16219 + 0.5705388888888888889e0 * t16221 + 0.13490888888888888889e1 * t16224;
    (t16210, t16215, t16217, t16219, t16221, t16224, t16226)
}
