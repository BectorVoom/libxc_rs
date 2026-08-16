//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1519;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1520;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta404<F: Float>(t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t10905: F, t4442: F, t4457: F, t775: F, t800: F, t1548: F, t2430: F, t240: F, t849: F, t14648: F, t2661: F, t2652: F, t4345: F, t10716: F, t4349: F, t10746: F, t10749: F, t10756: F, t10758: F, t2730: F, t2394: F, t2689: F, t4372: F, t4354: F, t9775: F, t14468: F, t828: F, t855: F, t221: F, t2675: F, t4343: F) -> (F, F, F, F, F, F, F) {
        let (t14817, t14820, t14823, t14825, t14829) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1519::<F>(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t4457, t775, t800, t1548, t2430);
        let (t14833, t14841) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1520::<F>(t240, t849, t14648, t775, t2661, t2652, t4345, t10716, t4349, t10746, t10749, t10756, t10758, t14817, t14820, t14823, t14825, t14829, t2730);
        let (t14843, t14846, t14850, t14853, t14857) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1521::<F>(t1548, t2394, t800, t2689, t4372, t4354, t9775, t14468, t828, t855, t221, t2675, t4343);
    (t14833, t14841, t14843, t14846, t14850, t14853, t14857)
}
