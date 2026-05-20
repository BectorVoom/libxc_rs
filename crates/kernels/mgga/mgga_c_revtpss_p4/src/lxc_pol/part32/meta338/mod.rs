//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1263;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta338<F: Float>(t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F, t1868: F, t4010: F, t1353: F, t2661: F, t550: F, t5658: F, t543: F, t3992: F, t5610: F, t9775: F, t1889: F, t9779: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13760, t13762, t13763, t13765, t13767, t13768) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1263::<F>(t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240, t1868, t4010);
        let (t13769, t13772, t13775, t13778, t13779, t13781) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1264::<F>(t1353, t13768, t13767, t2661, t550, t5658, t543, t3992, t5610, t9775, t1889, t9779);
    (t13760, t13762, t13763, t13765, t13769, t13772, t13775, t13778, t13779, t13781)
}
