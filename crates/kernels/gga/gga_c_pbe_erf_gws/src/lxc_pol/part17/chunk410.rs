//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 410/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk410<F: Float>(t1435: F, t1328: F, t1330: F, t1394: F, t1398: F, t1401: F, t1424: F, t1427: F, t1429: F, t1431: F, t1433: F, t408: F, t413: F) -> (F, F, F) {
    let t1436 = F::new(0.36623110073506319882e-3) * t1435;
    let t1437 = -t1394 - t1398 - t1401 + t1328 + t1424 + t1427 - t1429 - t1431 + t1433 + t1330 - t1436;
    let t1438 = t408 * t413;
    (t1436, t1437, t1438)
}
