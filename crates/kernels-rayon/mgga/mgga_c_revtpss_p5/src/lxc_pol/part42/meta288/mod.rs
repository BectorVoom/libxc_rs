//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1046;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta288(t2482: f64, t27: f64, t2719: f64, t820: f64, t843: f64, t821: f64, t235: f64, t231: f64, t2723: f64, t2710: f64, t826: f64, t9732: f64, t234: f64, t2735: f64, t10631: f64, t808: f64, t2699: f64, t798: f64, t802: f64, t159: f64, t853: f64, t216: f64, t2729: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10850, t10858, t10867, t10868, t10871, t10885) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1046(t2482, t27, t2719, t820, t843, t821, t235, t231, t2723, t2710, t826, t9732);
        let (t10886, t10888, t10890, t10891, t10900, t10905) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1047(t234, t2735, t10631, t808, t2699, t798, t802, t159, t853, t216, t2729, t794);
    (t10850, t10858, t10867, t10868, t10871, t10885, t10886, t10888, t10890, t10891, t10900, t10905)
}
