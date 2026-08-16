//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1336;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1337;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta373(t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t3923: f64, t9994: f64, t5673: f64, t5674: f64, t5697: f64, t9962: f64, t5701: f64, t13778: f64, t13779: f64, t13781: f64, t13786: f64, t13793: f64, t13797: f64, t13798: f64, t3934: f64, t5671: f64, t9735: f64, t4004: f64, t9840: f64, t1868: f64, t3829: f64, t828: f64, t9942: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13801, t13804, t13805, t13807, t13810) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1336(t5651, t808, t9736, t241, t820, t9991, t3923, t9994, t5673, t5674, t5697, t9962);
        let t13814 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1337(t5701, t9962, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t13807, t13810, t3934, t5671, t9735);
        let (t13817, t13821, t13824, t13826, t13829, t13832) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1338(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
    (t13805, t13807, t13814, t13817, t13821, t13824, t13826, t13829, t13832)
}
