//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk900;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk901;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk902;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk903;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta196(t38: f64, t4217: f64, t1469: f64, t2299: f64, t4186: f64, t633: f64, t2306: f64, t637: f64, t606: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t4182: f64, t4188: f64, t4191: f64, t4196: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t1497: f64, t2242: f64, t2247: f64, t4171: f64, t4173: f64, t4178: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t1501: f64, t670: f64, t94: f64, t1310: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4218, t4227, t4232, t4237, t4238, t4241) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk900(t38, t4217, t1469, t2299, t4186, t633, t2306, t637, t606, t77, t1471, t1487, t1494, t4182, t4188, t4191, t4196, t608, t628, t641, t71, t85);
        let (t4245, t4246) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk901(t5, t1497, t2242, t2247, t4171, t4173, t4178, t4241, t603, t644, t91, t117);
        let t4248 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk902(t116, t1501);
        let t4254 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk903(t670, t94);
        let t4257 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk904(t1310, t1518);
    (t4218, t4227, t4232, t4237, t4238, t4241, t4245, t4246, t4248, t4254, t4257)
}
