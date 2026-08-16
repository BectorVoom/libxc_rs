//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2237;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta595(t1012: f64, t23868: f64, t1011: f64, t1041: f64, t1063: f64, t11246: f64, t11256: f64, t11630: f64, t11927: f64, t15707: f64, t15823: f64, t15932: f64, t1671: f64, t19659: f64, t19697: f64, t23630: f64, t23635: f64, t23643: f64, t23823: f64, t23830: f64, t23834: f64, t23839: f64, t23844: f64, t23848: f64, t23852: f64, t23859: f64, t23863: f64, t3127: f64, t4837: f64, t4879: f64, t6263: f64, t6302: f64, t6308: f64, t6312: f64, t11822: f64, t22688: f64, t11827: f64, t23481: f64, t247: f64, t3182: f64, t1592: f64, t19675: f64, t1042: f64, t11660: f64, t1469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23869, t23872) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2237(t1012, t23868, t1011, t1041, t1063, t11246, t11256, t11630, t11927, t15707, t15823, t15932, t1671, t19659, t19697, t23630, t23635, t23643, t23823, t23830, t23834, t23839, t23844, t23848, t23852, t23859, t23863, t3127, t4837, t4879, t6263, t6302, t6308, t6312);
        let (t23873, t23874, t23877, t23878, t23886, t23891, t23892, t23898) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2238(t11822, t22688, t1012, t11827, t23481, t247, t3182, t1592, t19675, t1042, t11660, t1469);
    (t23869, t23872, t23873, t23874, t23877, t23878, t23886, t23891, t23892, t23898)
}
