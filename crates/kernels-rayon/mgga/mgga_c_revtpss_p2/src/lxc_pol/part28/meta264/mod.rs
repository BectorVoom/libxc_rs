//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1184;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1185;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1186;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta264(t1976: f64, t988: f64, t7145: f64, t1981: f64, t3056: f64, t7143: f64, t999: f64, t1071: f64, t1982: f64, t3268: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7146, t7147, t7150) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1183(t1976, t988, t7145, t1981, t3056);
        let t7151 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1184(t7143, t7150);
        let (t7152, t7153, t7156) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1185(t1976, t999, t7145, t1071, t1982);
        let t7159 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1186(t1982, t7143);
        let t7160 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1187(t3268, t359);
    (t7146, t7147, t7150, t7151, t7152, t7153, t7156, t7159, t7160)
}
