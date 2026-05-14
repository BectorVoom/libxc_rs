//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 991/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk991<F: Float>(t14046: F, t4029: F, t3139: F, t6178: F, t4028: F, t1184: F, t2212: F, t14004: F, t14008: F, t14012: F, t14016: F, t14018: F, t14020: F, t14026: F, t14030: F, t14032: F, t14036: F, t14038: F, t14040: F, t14043: F) -> (F, F, F) {
    let t14047 = t14046 * t4029;
    let t14048 = 7.0 / 144.0 * t14047;
    let t14049 = t3139 * t6178;
    let t14050 = t4028 * t14049;
    let t14052 = t1184 * t2212;
    let t14054 = t14004 / 96.0 - t14008 / 768.0 + t14012 / 96.0 - t14016 / 96.0 + t14018 / 96.0 + t14020 / 96.0 - t14026 - t14030 - t14032 / 192.0 + t14036 / 256.0 + t14038 / 24.0 - t14040 / 48.0 + t14043 + t14048 - t14050 / 96.0 + t14052 / 16.0;
    (t14047, t14049, t14054)
}
