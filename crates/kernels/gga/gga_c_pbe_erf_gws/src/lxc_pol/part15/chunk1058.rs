//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1058/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1058<F: Float>(t2319: F, t3295: F, t1123: F, t6303: F, t2255: F, t1105: F, t904: F, t2258: F, t1153: F, t9521: F, t8827: F, t3223: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t9601 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2319 * t3295;
    let t9603 = t1123 * t6303;
    let t9604 = t2255 * t9603;
    let t9607 = t1105 * param_a_c;
    let t9608 = t904 * t9607;
    let t9609 = t9608 * t2258;
    let t9612 = t1153 * t9521;
    let t9615 = t904 * t8827;
    let t9616 = t9615 * t3223;
    (t9601, t9603, t9604, t9607, t9609, t9612, t9616)
}
