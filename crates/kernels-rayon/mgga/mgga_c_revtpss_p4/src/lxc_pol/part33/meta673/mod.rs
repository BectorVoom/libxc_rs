//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2203;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta673(t27833: f64, t7901: f64, t2014: f64, t28020: f64, t5542: f64, t1450: f64, t21969: f64, t7237: f64, t28167: f64, t35669: f64, t5627: f64, t29996: f64, t7235: f64, t22483: f64, t7312: f64, t28172: f64, t28176: f64, t29498: f64, t94345: f64, t29583: f64, t2322: f64, t30128: f64, t4254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109112, t109117, t109121, t109124, t109126) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2203(t27833, t7901, t2014, t28020, t5542, t1450, t21969, t7237, t28167, t35669, t5627, t29996, t7235);
        let (t109128, t109135, t109138, t109140, t109142, t109144) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2204(t2014, t22483, t7312, t28172, t28176, t29498, t94345, t29583, t7235, t2322, t30128, t4254);
    (t109112, t109117, t109121, t109124, t109126, t109128, t109135, t109138, t109140, t109142, t109144)
}
