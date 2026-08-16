//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2038/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2038(t107974: f64, t108002: f64, t108005: f64, t108021: f64, t108033: f64, t108043: f64, t110150: f64, t110154: f64, t110165: f64, t1711: f64, t1940: f64, t20256: f64, t2071: f64, t2403: f64, t26425: f64, t26585: f64, t27800: f64, t28291: f64, t28456: f64, t29946: f64, t29967: f64, t7432: f64, t7862: f64, t95511: f64) -> f64 {
    let t110883 = -3.0_f64 * t28291 * t108002 - t1940 * t7432 * t108043 / 2.0_f64 + 6.0_f64 * t26425 * t107974 + t1940 * t28456 * t1711 - t1940 * t7432 * t108005 / 2.0_f64 + 2.0_f64 * t110165 * t27800 - t110150 - 3.0_f64 * t95511 * t29946 + 6.0_f64 * t28291 * t108033 - t1940 * t26585 * t29967 - t1940 * t7432 * t108021 / 2.0_f64 + t110154 + t1940 * t2071 * t20256 / 2.0_f64 + 3.0_f64 * t2403 * t28456 * t7862;
    t110883
}
