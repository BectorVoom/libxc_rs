//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1290/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1290(t128869: f64, t128871: f64, t128874: f64, t128876: f64, t128878: f64, t128880: f64, t128882: f64, t128891: f64, t27060: f64, t28704: f64, t28711: f64, t28727: f64, t29432: f64, t7586: f64, t7984: f64, t8764: f64) -> f64 {
    let t131018 = -2.0_f64 * t27060 * t7984 - 2.0_f64 * t28704 * t7586 - 2.0_f64 * t28711 * t7586 - t28727 * t8764 - 2.0_f64 * t29432 * t7984 + t128869 - t128871 - t128874 - t128876 - t128878 - t128880 - t128882 - t128891;
    t131018
}
