//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 404/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk404<F: Float>(t1378: F, t20: F, t161: F, t703: F, t1369: F, t1372: F, t1375: F, t696: F, t697: F) -> (F, F, F) {
    let t1379 = t1378 * t20;
    let t1380 = t703 * t161;
    let t1383 = t1369 / F::new(2.0) + F::new(0.627e-1) * t1372 * t697 - F::new(0.418e-1) * t696 * t1375 + F::new(0.786258e-2) * t1379 * t1380;
    (t1379, t1380, t1383)
}
