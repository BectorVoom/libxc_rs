//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1621;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1622;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1623;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta431<F: Float>(t1655: F, t697: F, t1011: F, t372: F, t4806: F, t15702: F, t15688: F, t3299: F, t1043: F, t905: F, t606: F, t3155: F, t15691: F, t1047: F, t1063: F, t11656: F, t11977: F, t15700: F, t16190: F, t16196: F, t16201: F, t16205: F, t16210: F, t16218: F, t1671: F, t3169: F, t4825: F, t4869: F, t15625: F, t15676: F, t15722: F, t15755: F, t15779: F, t15814: F, t15855: F, t15913: F, t15949: F, t15991: F, t16034: F, t16073: F, t16114: F, t16136: F, t16189: F, t225: F, t385: F, t1096: F, t4772: F, t1079: F, t1651: F, t3269: F, t3270: F, t5015: F, t1073: F, t1076: F, t11190: F, t11224: F, t15579: F, t15886: F, t1647: F, t1652: F, t3047: F, t3052: F, t3063: F, t3261: F, t342: F, t386: F, t4743: F, t4758: F, t4764: F, t4932: F, t4941: F, t4947: F, t989: F, t995: F) -> (F, F, F, F, F, F, F, F) {
        let (t16219, t16220, t16223, t16226, t16229) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1621::<F>(t1655, t697, t1011, t372, t4806, t15702, t15688, t3299, t1043, t905, t606, t3155);
        let (t16230, t16233) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1622::<F>(t15691, t16229, t1047, t1063, t11656, t11977, t15700, t16190, t16196, t16201, t16205, t16210, t16218, t16220, t16223, t16226, t1671, t3169, t4825, t4869);
        let t16237 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1623::<F>(t15625, t15676, t15722, t15755, t15779, t15814, t15855, t15913, t15949, t15991, t16034, t16073, t16114, t16136, t16189, t16233);
        let (t16243, t16249, t16255, t16272) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1624::<F>(t16237, t225, t385, t1096, t4772, t1079, t1651, t3269, t3270, t5015, t1073, t1076, t11190, t11224, t15579, t15886, t1647, t1652, t3047, t3052, t3063, t3261, t342, t386, t4743, t4758, t4764, t4932, t4941, t4947, t989, t995);
    (t16219, t16223, t16230, t16237, t16243, t16249, t16255, t16272)
}
