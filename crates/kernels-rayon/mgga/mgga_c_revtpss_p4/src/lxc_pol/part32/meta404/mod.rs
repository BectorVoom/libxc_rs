//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1393;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1394;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta404(t18330: f64, t18343: f64, t18361: f64, t18405: f64, t18454: f64, t18489: f64, t18524: f64, t18654: f64, t225: f64, t6048: f64, t886: f64, t11008: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64, t10519: f64, t10539: f64, t14498: f64, t14506: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t14525: f64, t14533: f64, t14539: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t5978: f64, t820: f64, t837: f64, t233: f64, t6041: f64, t869: f64, t689: f64, t6016: f64, t822: f64, t6022: f64, t72: f64, t686: f64, t10530: f64, t10645: f64, t10647: f64, t10651: f64, t14558: f64, t14564: f64, t14570: f64, t18616: f64, t18632: f64, t213: f64, t234: f64, t4504: f64, t4526: f64, t6017: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18657, t18658, t18663) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1393(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654, t225, t6048, t886, t11008);
        let (t18677, t18681, t18687) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1394(t251, t5977, t1558, t1568, t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18699, t18722) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1395(t233, t6041, t869, t689, t251, t6016, t822, t6022, t72, t686, t10530, t10645, t10647, t10651, t14558, t14564, t14570, t18616, t18632, t18657, t213, t234, t2815, t4424, t4494, t4504, t4514, t4526, t6017, t820, t837, t879);
    (t18657, t18658, t18663, t18677, t18681, t18687, t18699, t18722)
}
