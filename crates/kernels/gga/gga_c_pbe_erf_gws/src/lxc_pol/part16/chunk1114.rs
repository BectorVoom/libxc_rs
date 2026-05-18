//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1114/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1114<F: Float>(t2367: F, t4083: F, t1205: F, t2074: F, t2376: F, t2409: F, t14004: F, t14008: F, t14012: F, t14016: F, t14018: F, t14020: F, t14025: F, t14029: F, t14032: F, t14036: F, t14038: F, t14040: F, t14042: F, t14047: F, t14050: F, t14052: F) -> (F, F, F, F) {
    let t14198 = t2367 * t4083;
    let t14200 = t1205 * t2074;
    let t14202 = t2409 * t2376 * t14200;
    let t14222 = t14004 / F::new(48.0) - t14008 / F::new(384.0) + t14012 / F::new(48.0) - t14016 / F::new(48.0) + t14018 / F::new(48.0) + t14020 / F::new(48.0) - F::new(7.0) / F::new(72.0) * t14025 - F::new(7.0) / F::new(288.0) * t14029 - t14032 / F::new(96.0) + t14036 / F::new(128.0) + t14038 / F::new(12.0) - t14040 / F::new(24.0) + F::new(7.0) / F::new(36.0) * t14042 + F::new(7.0) / F::new(72.0) * t14047 - t14050 / F::new(48.0) + t14052 / F::new(8.0);
    (t14198, t14200, t14202, t14222)
}
