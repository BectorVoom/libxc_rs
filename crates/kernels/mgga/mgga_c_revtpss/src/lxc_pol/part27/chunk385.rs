//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 385/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk385<F: Float>(t1368: F, t1370: F, t1372: F, t1378: F, t1383: F, t1388: F, t1401: F, t1407: F, t1410: F, t1416: F) -> F {
    let t1419 = -t1368 - t1370 * t1372 / F::new(48.0) - t1378 + t1383 - F::new(0.21437009059034868486e-3) * t1388 * t1401 - t1407 - F::new(0.85748036236139473944e-3) * t1410 * t1416;
    t1419
}
