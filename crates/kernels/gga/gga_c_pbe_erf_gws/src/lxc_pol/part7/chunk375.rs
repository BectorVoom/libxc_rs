//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 375/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk375<F: Float>(t1235: F, t103: F, t1251: F, t1: F, t120: F, t485: F) -> (F, F, F, F) {
    let t1508 = param_hyb_omega_0 * t1235;
    let t1509 = t1508 * t103;
    let t1511 = F::new(0.32478055555555555555e0) * t1509 * t1251;
    let t1513 = t485 * t120 * t1;
    (t1508, t1509, t1511, t1513)
}
