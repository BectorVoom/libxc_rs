//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1457;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1458;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta262<F: Float>(t1414: F, t828: F, t9628: F, t221: F, t3889: F, t3979: F, t3978: F, t1408: F, t2482: F, t596: F, t3981: F, t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t816: F, t3995: F, t2681: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9757, t9761, t9762, t9765) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1457::<F>(t1414, t828, t9628, t221, t3889, t3979, t3978, t1408, t2482, t596);
        let (t9766, t9768, t9769, t9770, t9771, t9775) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1458::<F>(t3981, t9765, t3923, t550, t543, t3992, t2661, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1459::<F>(t3995, t9775, t1408, t2681, t820);
    (t9757, t9761, t9762, t9765, t9766, t9768, t9769, t9770, t9771, t9775, t9776, t9779)
}
