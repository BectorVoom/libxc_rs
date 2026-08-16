//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1040;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta284(t2665: f64, t9775: f64, t2681: f64, t820: f64, t849: f64, t857: f64, t240: f64, t2719: f64, t2735: f64, t2783: f64, t2664: f64, t808: f64, t2693: f64, t2710: f64, t2713: f64, t810: f64, t9784: f64, t9789: f64, t235: f64, t2453: f64, t9794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10719, t10722, t10723, t10726, t10744, t10745) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1040(t2665, t9775, t2681, t820, t849, t857, t240, t2719, t2735, t2783, t2664, t808);
        let (t10746, t10749, t10756, t10758, t10760, t10761) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1041(t10744, t10745, t2693, t2710, t2713, t810, t9784, t9789, t235, t2783, t2453, t2664, t9794);
    (t10719, t10722, t10723, t10726, t10744, t10746, t10749, t10756, t10758, t10760, t10761)
}
