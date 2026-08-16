//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk847;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk848;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk849;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk850;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk851;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta156(t1343: f64, t1353: f64, t1450: f64, t198: f64, t3871: f64, t3873: f64, t3889: f64, t4025: f64, t4027: f64, t4031: f64, t4033: f64, t4035: f64, t4037: f64, t4040: f64, t4042: f64, t4135: f64, t4139: f64, t4140: f64, t4144: f64, t4147: f64, t532: f64, t3868: f64, t118: f64, t1310: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t116: f64, t2327: f64, t117: f64, t2371: f64, t1459: f64, t1461: f64, t572: f64, t573: f64, t670: f64, t94: f64, param_d: f64, t241: f64, t2719: f64, t820: f64, t243: f64, t72: f64, t245: f64, t2723: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4150 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk847(t1343, t1353, t1450, t198, t3871, t3873, t3889, t4025, t4027, t4031, t4033, t4035, t4037, t4040, t4042, t4135, t4139, t4140, t4144, t4147, t532);
        let (t4151, t4153) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk848(t3868, t4150, t118, t1310, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t508, t511, t569, t649, t651, t671);
        let (t4154, t4158, t4162, t4165, t4168, t4254) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk849(t3, t4153, t116, t2327, t117, t2371, t1459, t1461, t572, t573, t670, t94, param_d);
        let t4362 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk850(t241, t2719, t820);
        let t4364 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk851(t243, t72, t245);
        let t4366 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk852(t2723, t836);
    (t4151, t4153, t4154, t4158, t4162, t4165, t4168, t4254, t4362, t4364, t4366)
}
