//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta194 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk950;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk951;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk952;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk953;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta194<F: Float>(t38: F, t4217: F, t1469: F, t2299: F, t4186: F, t633: F, t2306: F, t637: F, t606: F, t77: F, t1471: F, t1487: F, t1494: F, t4182: F, t4188: F, t4191: F, t4196: F, t608: F, t628: F, t641: F, t71: F, t85: F, t5: F, t1497: F, t2242: F, t2247: F, t4171: F, t4173: F, t4178: F, t603: F, t644: F, t91: F, t117: F, t116: F, t1501: F, t670: F, t94: F, t1310: F, t1518: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4218, t4227, t4232, t4237, t4238, t4241) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk950::<F>(t38, t4217, t1469, t2299, t4186, t633, t2306, t637, t606, t77, t1471, t1487, t1494, t4182, t4188, t4191, t4196, t608, t628, t641, t71, t85);
        let (t4245, t4246) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk951::<F>(t5, t1497, t2242, t2247, t4171, t4173, t4178, t4241, t603, t644, t91, t117);
        let t4248 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk952::<F>(t116, t1501);
        let t4254 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk953::<F>(t670, t94);
        let t4257 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk954::<F>(t1310, t1518);
    (t4218, t4227, t4232, t4237, t4238, t4241, t4245, t4246, t4248, t4254, t4257)
}
