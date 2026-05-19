//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1023/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1023<F: Float>(t414: F, t4832: F, t1399: F, t4749: F, t409: F, t4601: F, t1425: F, t1438: F, t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t7236: F, t7271: F) -> (F, F, F, F, F) {
    let t18534 = t414 * t4832;
    let t18535 = F::new(16.0) * t18534;
    let t18536 = t1399 * t4749;
    let t18537 = F::cast_from(0.4155781415850207192e3_f64) * t18536;
    let t18538 = t409 * t4601;
    let t18539 = F::new(48.0) * t18538;
    let t18540 = t1438 * t1425;
    let t18541 = F::new(384.0) * t18540;
    let t18552 = -F::cast_from(0.28769444444444444444e1_f64) * t18486 + F::cast_from(0.27618666666666666667e2_f64) * t18488 - F::cast_from(0.10229135802469135803e2_f64) * t18491 + F::cast_from(0.89504938271604938273e1_f64) * t18494 + F::cast_from(0.31310740740740740741e1_f64) * t7271 + F::new(0.366775e-1) * t18500 - F::new(0.58684e0) * t18502 + F::cast_from(0.65204444444444444445e0_f64) * t18504 + F::cast_from(0.5705388888888888889e0_f64) * t18506 + F::cast_from(0.13490888888888888889e1_f64) * t7236;
    (t18535, t18537, t18539, t18541, t18552)
}
