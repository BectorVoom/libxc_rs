//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1156;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1157;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1158;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1159;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1160;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta176<F: Float>(t565: F, t1343: F, t1353: F, t1450: F, t198: F, t3871: F, t3873: F, t3889: F, t4025: F, t4027: F, t4031: F, t4033: F, t4035: F, t4037: F, t4040: F, t4042: F, t4135: F, t4139: F, t4140: F, t4144: F, t532: F, t3868: F, t118: F, t1310: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t116: F, t2327: F, t117: F, t2371: F, t1459: F, t1461: F, t572: F, t573: F, t2219: F, t2223: F, t2226: F, t2230: F, t2233: F, t2239: F, param_d: F, t1466: F, t602: F, t1497: F, t644: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1156::<F>(t565);
        let t4150 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1157::<F>(t1343, t1353, t1450, t198, t3871, t3873, t3889, t4025, t4027, t4031, t4033, t4035, t4037, t4040, t4042, t4135, t4139, t4140, t4144, t4147, t532);
        let (t4151, t4153) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1158::<F>(t3868, t4150, t118, t1310, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t508, t511, t569, t649, t651, t671);
        let (t4154, t4158, t4162, t4165, t4168, t4171) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1159::<F>(t3, t4153, t116, t2327, t117, t2371, t1459, t1461, t572, t573, t2219, t2223, t2226, t2230, t2233, t2239, param_d);
        let t4173 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1160::<F>(t1466, t602);
        let t4178 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1161::<F>(t1497, t644);
    (t4146, t4147, t4151, t4153, t4154, t4158, t4162, t4165, t4168, t4171, t4173, t4178)
}
