//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2446;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2447;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2448;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta583(t18657: f64, t225: f64, t6048: f64, t886: f64, t11008: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64, t10519: f64, t10539: f64, t14498: f64, t14506: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t14525: f64, t14533: f64, t14539: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t5978: f64, t820: f64, t837: f64, t233: f64, t6041: f64, t869: f64, t689: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18658, t18662, t18663, t18677) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2446(t18657, t225, t6048, t886, t11008, t251, t5977);
        let t18681 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2447(t1558, t1568);
        let t18687 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2448(t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t18677, t18681, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18688, t18689, t18690, t18699) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2449(t233, t6041, t869, t689, t251, t6016);
    (t18658, t18662, t18663, t18677, t18681, t18687, t18688, t18689, t18690, t18699)
}
