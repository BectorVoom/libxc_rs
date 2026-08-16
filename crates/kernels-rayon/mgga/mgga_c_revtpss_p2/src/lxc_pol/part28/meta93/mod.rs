//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk591;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk592;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk593;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk594;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk595;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta93(t1962: f64, t892: f64, t30: f64, t1940: f64, t343: f64, t43: f64, t136: f64, t359: f64, sigma0: f64, t365: f64, t351: f64, t348: f64, t375: f64, t225: f64, t385: f64, t338: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1963 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk591(t1962, t892);
        let (t1966, t1967, t1968, t1971) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk592(t1963, t30, t1940, t343, t43, t136, t359, sigma0);
        let t1972 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk593(t1971, t365);
        let (t1973, t1976) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk594(t1972, t351, t1968, t348, t375);
        let (t1977, t1978) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk595(t1976, t225, t385);
        let (t1981, t1982) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk596(t338, t993);
    (t1963, t1966, t1967, t1968, t1971, t1972, t1973, t1976, t1977, t1978, t1981, t1982)
}
