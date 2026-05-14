//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 921/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk921<F: Float>(t414: F, t4832: F, t1399: F, t4749: F, t409: F, t4601: F, t1425: F, t1438: F, t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t7236: F, t7271: F) -> (F, F, F, F, F) {
    let t18534 = t414 * t4832;
    let t18535 = 16.0 * t18534;
    let t18536 = t1399 * t4749;
    let t18537 = 0.4155781415850207192e3 * t18536;
    let t18538 = t409 * t4601;
    let t18539 = 48.0 * t18538;
    let t18540 = t1438 * t1425;
    let t18541 = 384.0 * t18540;
    let t18552 = -0.28769444444444444444e1 * t18486 + 0.27618666666666666667e2 * t18488 - 0.10229135802469135803e2 * t18491 + 0.89504938271604938273e1 * t18494 + 0.31310740740740740741e1 * t7271 + 0.366775e-1 * t18500 - 0.58684e0 * t18502 + 0.65204444444444444445e0 * t18504 + 0.5705388888888888889e0 * t18506 + 0.13490888888888888889e1 * t7236;
    (t18535, t18537, t18539, t18541, t18552)
}
