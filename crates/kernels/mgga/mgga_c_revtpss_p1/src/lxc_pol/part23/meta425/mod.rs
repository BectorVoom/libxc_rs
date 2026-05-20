//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1815;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1816;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1817;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta425<F: Float>(t18637: F, t2747: F, t4365: F, t10779: F, t14671: F, t6035: F, t10777: F, t14676: F, t18444: F, t4364: F, t837: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t18618: F, t18623: F, t18629: F, t18634: F, t2745: F, t4362: F, t825: F, t18330: F, t18343: F, t18361: F, t18405: F, t18454: F, t18489: F, t18524: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F, t1558: F, t1568: F, t10519: F, t10539: F, t14498: F, t14506: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t14539: F, t2815: F, t4424: F, t4494: F, t4514: F, t5978: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18639, t18643, t18644, t18647, t18651, t18654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814::<F>(t18637, t2747, t4365, t10779, t14671, t6035, t10777, t14676, t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t2745, t4362, t825);
        let t18657 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1815::<F>(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654);
        let (t18658, t18662, t18663, t18677) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1816::<F>(t18657, t225, t6048, t886, t11008, t251, t5977);
        let t18681 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1817::<F>(t1558, t1568);
        let t18687 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1818::<F>(t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t18677, t18681, t2815, t4424, t4494, t4514, t5978, t820, t837);
    (t18639, t18643, t18644, t18647, t18651, t18657, t18658, t18662, t18663, t18677, t18681, t18687)
}
