//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta46 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk315;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk316;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk317;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk318;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk319;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk320;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk321;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk322;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta46(t270: f64, t283: f64, t66: f64, t342: f64, t378: f64, t384: f64, t225: f64, t359: f64, t1032: f64, t1035: f64, t355: f64, t357: f64, t389: f64, t268: f64, t404: f64, t900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1065 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk315(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk316(t1065, t66);
        let t1076 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk317(t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk318(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk319(t359, t378);
        let t1086 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk320(t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk321(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk322(t355, t357);
        let (t1102, t1118) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk323(t389, t268, t404, t900);
    (t1065, t1066, t1076, t1077, t1078, t1079, t1082, t1086, t1087, t1089, t1102, t1118)
}
