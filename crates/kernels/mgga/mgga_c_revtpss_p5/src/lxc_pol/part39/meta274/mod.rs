//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1010;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta274<F: Float>(t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t596: F, t816: F, t3995: F, t1408: F, t2681: F, t820: F, t1416: F, t124: F, t2237: F, t800: F, t1376: F, t123: F, t125: F, t2452: F, t9720: F, t235: F, t4086: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9768, t9771, t9775, t9776, t9779) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1010::<F>(t3923, t550, t543, t3992, t2661, t212, t225, t596, t816, t3995, t1408, t2681, t820);
        let (t9780, t9784, t9786, t9789, t9791, t9792) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1011::<F>(t1416, t9779, t124, t212, t2237, t800, t1376, t123, t125, t2452, t9720, t235, t4086);
    (t9768, t9771, t9775, t9776, t9779, t9780, t9784, t9786, t9789, t9791, t9792)
}
