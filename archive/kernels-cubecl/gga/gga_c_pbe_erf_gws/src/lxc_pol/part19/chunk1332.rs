//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1332/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1332<F: Float>(t1118: F, t1133: F, t361: F, t3223: F, t50998: F, t12239: F, t14121: F, t11409: F, t3965: F, t12250: F, t3959: F, t2409: F, t35433: F) -> (F, F, F, F, F) {
    let t57384 = t361 * t1118 * t1133;
    let t57386 = t50998 * t57384 * t3223;
    let t57390 = t14121 * t12239;
    let t57393 = t3965 * t11409;
    let t57395 = t3959 * t12250;
    let t57398 = t3959 * t2409 * t35433;
    (t57386, t57390, t57393, t57395, t57398)
}
