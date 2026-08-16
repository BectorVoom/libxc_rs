//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk591;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk592;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk593;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk594;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk595;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk596;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk597;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta93(t2023: f64, t561: f64, t1955: f64, t555: f64, t1032: f64, t1426: f64, t2022: f64, t545: f64, t213: f64, t532: f64, t1450: f64, t2014: f64, t117: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2024, t2027) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk591(t2023, t561, t1955, t555);
        let t2028 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk592(t1032, t1426);
        let t2029 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk593(t2022, t545);
        let t2030 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk594(t2028, t2029);
        let t2033 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk595(t2024, t2027, t2030, t213);
        let t2034 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk596(t2033, t532);
        let t2035 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk597(t1450, t2034);
        let (t2036, t2042) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk598(t2014, t2035, t117, t1936);
    (t2024, t2027, t2028, t2029, t2030, t2033, t2034, t2035, t2036, t2042)
}
