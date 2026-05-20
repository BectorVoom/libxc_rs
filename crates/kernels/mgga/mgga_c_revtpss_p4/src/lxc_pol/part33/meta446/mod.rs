//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1630;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta446<F: Float>(t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t1214: F, t6587: F, t1211: F, t6744: F, t1277: F, t1294: F, t6573: F, t1774: F, t5245: F, t1210: F, t1215: F, t1295: F, t1775: F, t18037: F, t3561: F, t3567: F, t3572: F, t3732: F, t5225: F, t5237: F, t5251: F, t5417: F, t5429: F, t5498: F, t6580: F, t6745: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20692, t20697, t20700, t20703, t20704, t20710, t20714) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1630::<F>(t3801, t6748, t1209, t6695, t460, t1214, t6587, t1211, t6744, t1277, t1294, t6573);
        let (t20721, t20722, t20728, t20735) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1631::<F>(t1774, t5245, t1211, t1294, t6587, t1277, t1210, t1215, t1295, t1775, t18037, t20697, t20700, t20704, t20710, t20714, t3561, t3567, t3572, t3732, t5225, t5237, t5251, t5417, t5429, t5498, t6580, t6745);
    (t20692, t20703, t20704, t20710, t20714, t20721, t20722, t20728, t20735)
}
