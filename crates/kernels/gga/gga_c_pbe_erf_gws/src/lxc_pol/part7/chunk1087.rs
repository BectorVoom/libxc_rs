//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1087/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1087<F: Float>(t2096: F, t4343: F, t4339: F, t4544: F, t2074: F) -> (F, F, F) {
    let t19501 = t4343 * t2096;
    let t19502 = F::new(0.41076328840066666667e1) * t19501;
    let t19503 = t4544 * t4339;
    let t19504 = F::new(0.12654485932329694421e2) * t19503;
    let t19505 = t2074 * t2074;
    (t19502, t19504, t19505)
}
