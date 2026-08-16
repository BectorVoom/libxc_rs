//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta156 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk847;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk848;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk849;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk850;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk851;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta156<F: Float>(t1343: F, t1353: F, t1450: F, t198: F, t3871: F, t3873: F, t3889: F, t4025: F, t4027: F, t4031: F, t4033: F, t4035: F, t4037: F, t4040: F, t4042: F, t4135: F, t4139: F, t4140: F, t4144: F, t4147: F, t532: F, t3868: F, t118: F, t1310: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t116: F, t2327: F, t117: F, t2371: F, t1459: F, t1461: F, t572: F, t573: F, t670: F, t94: F, param_d: F, t241: F, t2719: F, t820: F, t243: F, t72: F, t245: F, t2723: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4150 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk847::<F>(t1343, t1353, t1450, t198, t3871, t3873, t3889, t4025, t4027, t4031, t4033, t4035, t4037, t4040, t4042, t4135, t4139, t4140, t4144, t4147, t532);
        let (t4151, t4153) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk848::<F>(t3868, t4150, t118, t1310, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t508, t511, t569, t649, t651, t671);
        let (t4154, t4158, t4162, t4165, t4168, t4254) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk849::<F>(t3, t4153, t116, t2327, t117, t2371, t1459, t1461, t572, t573, t670, t94, param_d);
        let t4362 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk850::<F>(t241, t2719, t820);
        let t4364 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk851::<F>(t243, t72, t245);
        let t4366 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk852::<F>(t2723, t836);
    (t4151, t4153, t4154, t4158, t4162, t4165, t4168, t4254, t4362, t4364, t4366)
}
