//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk587;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk588;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk589;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk590;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk591;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk592;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk593;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk594;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk595;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta91(t197: f64, t2013: f64, t1941: f64, t533: f64, t816: f64, t546: f64, t64: f64, t213: f64, t552: f64, t225: f64, t561: f64, t1955: f64, t555: f64, t1032: f64, t1426: f64, t545: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2014 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk587(t197, t2013);
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk588(t1941, t533, t816, t546, t64);
        let (t2019, t2022) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk589(t2018, t213, t552, t2016);
        let t2023 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk590(t2022, t225);
        let (t2024, t2027) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk591(t2023, t561, t1955, t555);
        let t2028 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk592(t1032, t1426);
        let t2029 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk593(t2022, t545);
        let t2030 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk594(t2028, t2029);
        let t2033 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk595(t2024, t2027, t2030, t213);
        let t2034 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk596(t2033, t532);
    (t2014, t2018, t2019, t2022, t2023, t2024, t2027, t2028, t2029, t2030, t2033, t2034)
}
