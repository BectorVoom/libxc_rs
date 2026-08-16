//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1476;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta401(t4292: f64, t94: f64, t1513: f64, t665: f64, t93: f64, t2339: f64, t625: f64, t655: f64, t10208: f64, t69: f64, t2195: f64, t2289: f64, t8312: f64, t8316: f64, t2340: f64, t8311: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27126, t28036, t28219, t31027) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1475(t4292, t94, t1513, t665, t93, t2339, t625);
        let t31032 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1476(t625, t655);
        let (t31035, t31134, t31135, t31137, t31139, t31142) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1477(t10208, t69, t2195, t2289, t31027, t8312, t31032, t8316, t2340, t8311, t661, t665);
    (t27126, t28036, t28219, t31027, t31032, t31035, t31134, t31135, t31137, t31139, t31142)
}
