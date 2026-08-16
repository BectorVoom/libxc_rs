//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1046;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1047;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta230(t6173: f64, t954: f64, t2970: f64, t6157: f64, t2974: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t324: f64, t1633: f64, t973: f64, t2994: f64, t3001: f64, t4620: f64, t6114: f64, t6121: f64, t6127: f64, t6129: f64, t6133: f64, t6136: f64, t6139: f64, t3014: f64, t1622: f64, t1634: f64, t2943: f64, t2968: f64, t2987: f64, t3012: f64, t311: f64, t4647: f64, t4685: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6152: f64, t6158: f64, t946: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6174, t6177, t6184, t6185, t6189) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1046(t6173, t954, t2970, t6157, t2974, t4571, t6094, t6098, t6102, t324, t1633);
        let (t6190, t6205) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1047(t6189, t973, t2994, t3001, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
        let (t6206, t6209, t6212) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1048(t6205, t973, t3014, t6189, t1622, t1634, t2943, t2968, t2987, t3012, t311, t4647, t4685, t6106, t6108, t6112, t6144, t6147, t6152, t6158, t6174, t6177, t6185, t6190, t946, t965);
    (t6174, t6177, t6184, t6185, t6189, t6190, t6205, t6206, t6209, t6212)
}
