//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1441;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1442;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta382<F: Float>(t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t3923: F, t9994: F, t5673: F, t5674: F, t5697: F, t9962: F, t5701: F, t13778: F, t13779: F, t13781: F, t13786: F, t13793: F, t13797: F, t13798: F, t3934: F, t5671: F, t9735: F, t4004: F, t9840: F, t1868: F, t3829: F, t828: F, t9942: F, t5608: F, t5675: F, t9934: F, t2661: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13801, t13804, t13805, t13807, t13810) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1441::<F>(t5651, t808, t9736, t241, t820, t9991, t3923, t9994, t5673, t5674, t5697, t9962);
        let t13814 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1442::<F>(t5701, t9962, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t13807, t13810, t3934, t5671, t9735);
        let (t13817, t13821, t13824, t13826, t13829, t13832) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1443::<F>(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
    (t13805, t13807, t13814, t13817, t13821, t13824, t13826, t13829, t13832)
}
