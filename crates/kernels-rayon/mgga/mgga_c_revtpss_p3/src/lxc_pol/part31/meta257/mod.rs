//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta257(t7014: f64, t780: f64, t689: f64, t1950: f64, t786: f64, t789: f64, t159: f64, t793: f64, t218: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7015, t7017, t7018, t7020, t7021) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1138(t7014, t780, t689, t1950, t786, t789, t159, t793);
        let (t7024, t7025) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1139(t218, t7021, t816, t1941, t228);
        let (t7026, t7028) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1140(t7025, t802, t240, t64);
    (t7015, t7017, t7018, t7020, t7021, t7024, t7025, t7026, t7028)
}
