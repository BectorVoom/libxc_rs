//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta54 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk399;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk400;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk401;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk402;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk403;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk404;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk405;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk406;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta54(t1071: f64, t225: f64, t385: f64, t342: f64, t378: f64, t384: f64, t359: f64, t999: f64, t1032: f64, t1035: f64, t355: f64, t357: f64, t1043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1072, t1073) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk399(t1071, t225, t385);
        let t1076 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk400(t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk401(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk402(t359, t378);
        let t1083 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk403(t1082, t999);
        let t1086 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk404(t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk405(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk406(t355, t357);
        let t1090 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk407(t1043, t1089, t378);
    (t1072, t1073, t1076, t1077, t1078, t1079, t1082, t1083, t1086, t1087, t1089, t1090)
}
