//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 787/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk787<F: Float>(t164: F, t3013: F, t8048: F, t2519: F, t547: F, t2523: F, t1964: F, t992: F, t2030: F, t987: F, t475: F, t2932: F, t751: F, t2936: F, t1: F, t1098: F, t2057: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8474 = t3013 * t164;
    let t8477 = 0.63010814446282235668e-1 * t8048 * t164;
    let t8478 = t2519 * t547;
    let t8489 = 0.63010814446282235668e-1 * t2523 * t547;
    let t8490 = t992 * t1964;
    let t8496 = t987 * t2030;
    let t8497 = t475 * t8496;
    let t8502 = 0.39914113367515363646e-1 * t751 * t2932;
    let t8503 = t751 * t2936;
    let t8519 = t1098 * t2057 * t1;
    (t8474, t8477, t8478, t8489, t8490, t8497, t8502, t8503, t8519)
}
