//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1136;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1137;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1138;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta255(t651: f64, t7003: f64, t30: f64, t775: f64, t1949: f64, t212: f64, t780: f64, t689: f64, t1950: f64, t786: f64, t789: f64, t159: f64, t793: f64, t218: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7005, t7010, t7014) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1136(t651, t7003, t30, t775, t1949, t212);
        let (t7015, t7017, t7018, t7020, t7021) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1137(t7014, t780, t689, t1950, t786, t789, t159, t793);
        let (t7024, t7025) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1138(t218, t7021, t816, t1941, t228);
        let (t7026, t7028) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1139(t7025, t802, t240, t64);
    (t7005, t7010, t7014, t7015, t7017, t7018, t7020, t7021, t7024, t7025, t7026, t7028)
}
