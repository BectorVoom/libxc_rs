//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1296/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1296<F: Float>(t51421: F, t9512: F, t14007: F, t9570: F, t51222: F, t4023: F, t9179: F, t51215: F, t54019: F, t54021: F, t54024: F, t54026: F, t54027: F, t54029: F, t54031: F) -> F {
    let t54033 = t51421 * t9512;
    let t54035 = t14007 * t9570;
    let t54038 = F::new(35.0) / F::new(216.0) * t51222;
    let t54039 = t9179 * t4023;
    let t54041 = -t54019 / F::new(96.0) - t54021 / F::new(192.0) - t54024 / F::new(24.0) + t54026 - t54027 / F::new(24.0) - t54029 / F::new(24.0) - t54031 / F::new(192.0) + F::new(5.0) / F::new(96.0) * t54033 - t54035 / F::new(128.0) + F::new(7.0) / F::new(1152.0) * t51215 + t54038 + t54039 / F::new(48.0);
    t54041
}
