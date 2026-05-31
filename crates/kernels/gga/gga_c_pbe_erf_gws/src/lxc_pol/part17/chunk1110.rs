//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1110/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1110<F: Float>(t14046: F, t4029: F, t3139: F, t6178: F, t4028: F, t1184: F, t2212: F, t14004: F, t14008: F, t14012: F, t14016: F, t14018: F, t14020: F, t14026: F, t14030: F, t14032: F, t14036: F, t14038: F, t14040: F, t14043: F) -> (F, F, F) {
    let t14047 = t14046 * t4029;
    let t14048 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14047;
    let t14049 = t3139 * t6178;
    let t14050 = t4028 * t14049;
    let t14052 = t1184 * t2212;
    let t14054 = t14004 / F::cast_from(96.0_f64) - t14008 / F::cast_from(768.0_f64) + t14012 / F::cast_from(96.0_f64) - t14016 / F::cast_from(96.0_f64) + t14018 / F::cast_from(96.0_f64) + t14020 / F::cast_from(96.0_f64) - t14026 - t14030 - t14032 / F::cast_from(192.0_f64) + t14036 / F::cast_from(256.0_f64) + t14038 / F::cast_from(24.0_f64) - t14040 / F::cast_from(48.0_f64) + t14043 + t14048 - t14050 / F::cast_from(96.0_f64) + t14052 / F::cast_from(16.0_f64);
    (t14047, t14049, t14054)
}
