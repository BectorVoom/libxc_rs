//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1393;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1394;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta404<F: Float>(t18330: F, t18343: F, t18361: F, t18405: F, t18454: F, t18489: F, t18524: F, t18654: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F, t1558: F, t1568: F, t10519: F, t10539: F, t14498: F, t14506: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t14539: F, t2815: F, t4424: F, t4494: F, t4514: F, t5978: F, t820: F, t837: F, t233: F, t6041: F, t869: F, t689: F, t6016: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t10645: F, t10647: F, t10651: F, t14558: F, t14564: F, t14570: F, t18616: F, t18632: F, t213: F, t234: F, t4504: F, t4526: F, t6017: F, t879: F) -> (F, F, F, F, F, F, F, F) {
        let (t18657, t18658, t18663) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1393::<F>(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654, t225, t6048, t886, t11008);
        let (t18677, t18681, t18687) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1394::<F>(t251, t5977, t1558, t1568, t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18699, t18722) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1395::<F>(t233, t6041, t869, t689, t251, t6016, t822, t6022, t72, t686, t10530, t10645, t10647, t10651, t14558, t14564, t14570, t18616, t18632, t18657, t213, t234, t2815, t4424, t4494, t4504, t4514, t4526, t6017, t820, t837, t879);
    (t18657, t18658, t18663, t18677, t18681, t18687, t18699, t18722)
}
