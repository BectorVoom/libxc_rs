//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1815;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1816;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1817;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta425(t18637: f64, t2747: f64, t4365: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t14676: f64, t18444: f64, t4364: f64, t837: f64, t14894: f64, t14907: f64, t14925: f64, t14934: f64, t18527: f64, t18532: f64, t18618: f64, t18623: f64, t18629: f64, t18634: f64, t2745: f64, t4362: f64, t825: f64, t18330: f64, t18343: f64, t18361: f64, t18405: f64, t18454: f64, t18489: f64, t18524: f64, t225: f64, t6048: f64, t886: f64, t11008: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64, t10519: f64, t10539: f64, t14498: f64, t14506: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t14525: f64, t14533: f64, t14539: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t5978: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18639, t18643, t18644, t18647, t18651, t18654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814(t18637, t2747, t4365, t10779, t14671, t6035, t10777, t14676, t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t2745, t4362, t825);
        let t18657 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1815(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654);
        let (t18658, t18662, t18663, t18677) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1816(t18657, t225, t6048, t886, t11008, t251, t5977);
        let t18681 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1817(t1558, t1568);
        let t18687 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1818(t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t18677, t18681, t2815, t4424, t4494, t4514, t5978, t820, t837);
    (t18639, t18643, t18644, t18647, t18651, t18657, t18658, t18662, t18663, t18677, t18681, t18687)
}
