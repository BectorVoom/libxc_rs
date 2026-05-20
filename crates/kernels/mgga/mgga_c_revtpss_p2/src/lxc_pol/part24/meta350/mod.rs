//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1213;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta350<F: Float>(t1012: F, t23868: F, t1011: F, t1041: F, t1063: F, t11246: F, t11256: F, t11630: F, t11927: F, t15707: F, t15823: F, t15932: F, t1671: F, t19659: F, t19697: F, t23630: F, t23635: F, t23643: F, t23823: F, t23830: F, t23834: F, t23839: F, t23844: F, t23848: F, t23852: F, t23859: F, t23863: F, t3127: F, t4837: F, t4879: F, t6263: F, t6302: F, t6308: F, t6312: F, t11822: F, t22688: F, t11827: F, t23481: F, t247: F, t3182: F, t1592: F, t19675: F, t1042: F, t11660: F, t1469: F) -> (F, F, F, F, F, F, F, F, F) {
        let t23872 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1213::<F>(t1012, t23868, t1011, t1041, t1063, t11246, t11256, t11630, t11927, t15707, t15823, t15932, t1671, t19659, t19697, t23630, t23635, t23643, t23823, t23830, t23834, t23839, t23844, t23848, t23852, t23859, t23863, t3127, t4837, t4879, t6263, t6302, t6308, t6312);
        let (t23873, t23874, t23877, t23878, t23886, t23891, t23892, t23898) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1214::<F>(t11822, t22688, t1012, t11827, t23481, t247, t3182, t1592, t19675, t1042, t11660, t1469);
    (t23872, t23873, t23874, t23877, t23878, t23886, t23891, t23892, t23898)
}
