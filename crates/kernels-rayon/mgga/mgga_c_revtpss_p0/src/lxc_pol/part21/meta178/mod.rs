//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1109;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1110;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1111;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta178(t565: f64, t1343: f64, t1353: f64, t1450: f64, t198: f64, t3871: f64, t3873: f64, t3889: f64, t4025: f64, t4027: f64, t4031: f64, t4033: f64, t4035: f64, t4037: f64, t4040: f64, t4042: f64, t4135: f64, t4139: f64, t4140: f64, t4144: f64, t532: f64, t3868: f64, t118: f64, t1310: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t116: f64, t2327: f64, t117: f64, t2371: f64, t1459: f64, t1461: f64, t572: f64, t573: f64, t2219: f64, t2223: f64, t2226: f64, t2230: f64, t2233: f64, t2239: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1109(t565);
        let t4150 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1110(t1343, t1353, t1450, t198, t3871, t3873, t3889, t4025, t4027, t4031, t4033, t4035, t4037, t4040, t4042, t4135, t4139, t4140, t4144, t4147, t532);
        let (t4151, t4153) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1111(t3868, t4150, t118, t1310, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t508, t511, t569, t649, t651, t671);
        let (t4154, t4158, t4162, t4165, t4168, t4171) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1112(t3, t4153, t116, t2327, t117, t2371, t1459, t1461, t572, t573, t2219, t2223, t2226, t2230, t2233, t2239, param_d);
    (t4146, t4147, t4151, t4153, t4154, t4158, t4162, t4165, t4168, t4171)
}
