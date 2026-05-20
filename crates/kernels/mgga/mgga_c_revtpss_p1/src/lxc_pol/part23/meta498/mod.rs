//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1981;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta498<F: Float>(t1211: F, t20721: F, t1294: F, t6587: F, t1277: F, t1210: F, t1215: F, t1295: F, t1775: F, t18037: F, t20697: F, t20700: F, t20704: F, t20710: F, t20714: F, t3561: F, t3567: F, t3572: F, t3732: F, t5225: F, t5237: F, t5251: F, t5417: F, t5429: F, t5498: F, t6580: F, t6745: F, t1214: F, t6702: F, t3737: F, t17974: F, t5422: F, t6573: F) -> (F, F, F, F, F, F, F, F) {
        let (t20722, t20727, t20728, t20735) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1981::<F>(t1211, t20721, t1294, t6587, t1277, t1210, t1215, t1295, t1775, t18037, t20697, t20700, t20704, t20710, t20714, t3561, t3567, t3572, t3732, t5225, t5237, t5251, t5417, t5429, t5498, t6580, t6745);
        let (t20740, t20741, t20744, t20747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1982::<F>(t1214, t6702, t3737, t17974, t5422, t6573);
    (t20722, t20727, t20728, t20735, t20740, t20741, t20744, t20747)
}
