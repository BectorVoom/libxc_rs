//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2446;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2447;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2448;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta583<F: Float>(t18657: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F, t1558: F, t1568: F, t10519: F, t10539: F, t14498: F, t14506: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t14539: F, t2815: F, t4424: F, t4494: F, t4514: F, t5978: F, t820: F, t837: F, t233: F, t6041: F, t869: F, t689: F, t6016: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18658, t18662, t18663, t18677) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2446::<F>(t18657, t225, t6048, t886, t11008, t251, t5977);
        let t18681 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2447::<F>(t1558, t1568);
        let t18687 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2448::<F>(t10519, t10539, t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t18677, t18681, t2815, t4424, t4494, t4514, t5978, t820, t837);
        let (t18688, t18689, t18690, t18699) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2449::<F>(t233, t6041, t869, t689, t251, t6016);
    (t18658, t18662, t18663, t18677, t18681, t18687, t18688, t18689, t18690, t18699)
}
