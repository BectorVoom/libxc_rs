//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2038/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2038<F: Float>(t107974: F, t108002: F, t108005: F, t108021: F, t108033: F, t108043: F, t110150: F, t110154: F, t110165: F, t1711: F, t1940: F, t20256: F, t2071: F, t2403: F, t26425: F, t26585: F, t27800: F, t28291: F, t28456: F, t29946: F, t29967: F, t7432: F, t7862: F, t95511: F) -> F {
    let t110883 = -F::new(3.0) * t28291 * t108002 - t1940 * t7432 * t108043 / F::new(2.0) + F::new(6.0) * t26425 * t107974 + t1940 * t28456 * t1711 - t1940 * t7432 * t108005 / F::new(2.0) + F::new(2.0) * t110165 * t27800 - t110150 - F::new(3.0) * t95511 * t29946 + F::new(6.0) * t28291 * t108033 - t1940 * t26585 * t29967 - t1940 * t7432 * t108021 / F::new(2.0) + t110154 + t1940 * t2071 * t20256 / F::new(2.0) + F::new(3.0) * t2403 * t28456 * t7862;
    t110883
}
