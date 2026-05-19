//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 850/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk850<F: Float>(t1971: F, t6055: F, t366: F, t535: F, t1378: F, t1368: F, t19: F, t1339: F, t331: F, t4562: F, t551: F, t553: F) -> (F, F, F, F, F, F) {
    let t16446 = F::cast_from(0.2267957317922316773e-1_f64) * t6055 * t1971;
    let t16447 = t366 * t535;
    let t16449 = t16447 * t1378 * t1971;
    let t16451 = t1368 * t19;
    let t16454 = F::cast_from(0.29725654166942986832e-2_f64) * t1339 * t16451 * t1971;
    let t16457 = t331 * t4562 * t551 * t553;
    (t16446, t16447, t16449, t16451, t16454, t16457)
}
