//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2595;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2596;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta650(t3801: f64, t6748: f64, t1209: f64, t6695: f64, t460: f64, t1214: f64, t6587: f64, t1211: f64, t6744: f64, t1277: f64, t1294: f64, t6573: f64, t1774: f64, t5245: f64, t1210: f64, t1215: f64, t1295: f64, t1775: f64, t18037: f64, t3561: f64, t3567: f64, t3572: f64, t3732: f64, t5225: f64, t5237: f64, t5251: f64, t5417: f64, t5429: f64, t5498: f64, t6580: f64, t6745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20692, t20697, t20700, t20703) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2595(t3801, t6748, t1209, t6695, t460, t1214, t6587);
        let (t20704, t20710, t20714, t20721) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2596(t1211, t20703, t1214, t6744, t1277, t1294, t6573, t1774, t5245);
        let (t20722, t20728, t20735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2597(t1211, t20721, t1294, t6587, t1277, t1210, t1215, t1295, t1775, t18037, t20697, t20700, t20704, t20710, t20714, t3561, t3567, t3572, t3732, t5225, t5237, t5251, t5417, t5429, t5498, t6580, t6745);
    (t20692, t20697, t20700, t20703, t20704, t20710, t20714, t20721, t20722, t20728, t20735)
}
