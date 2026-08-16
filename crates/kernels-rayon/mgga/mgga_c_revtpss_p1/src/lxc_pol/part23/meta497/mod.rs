//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1978;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1979;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta497(t3801: f64, t6748: f64, t1209: f64, t6695: f64, t460: f64, t1214: f64, t6587: f64, t1211: f64, t6744: f64, t1277: f64, t1294: f64, t6573: f64, t1774: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20692, t20697) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1978(t3801, t6748, t1209, t6695);
        let (t20700, t20703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1979(t460, t6695, t1214, t6587);
        let (t20704, t20709, t20710, t20714, t20721) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1980(t1211, t20703, t1214, t6744, t1277, t1294, t6573, t1774, t5245);
    (t20692, t20697, t20700, t20703, t20704, t20709, t20710, t20714, t20721)
}
