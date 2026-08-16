//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2367;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2368;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2369;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta549(t13045: f64, t3601: f64, t17710: f64, t3720: f64, t1261: f64, t12784: f64, t17669: f64, t17674: f64, t17679: f64, t17684: f64, t17690: f64, t17693: f64, t17696: f64, t17700: f64, t17705: f64, t17709: f64, t3625: f64, t3708: f64, t5287: f64, t5331: f64, t5340: f64, t5407: f64, t3172: f64, t5303: f64, t17633: f64, t5352: f64, t1209: f64, t489: f64, t3623: f64, t370: f64, t1214: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17711, t17712, t17713, t17718) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2367(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
        let (t17720, t17721, t17723, t17724, t17727, t17728) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2368(t3172, t5303, t1261, t17633, t5352, t3720, t1209, t489, t3623, t370);
        let t17729 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2369(t17727, t17728);
        let t17730 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2370(t1214, t606);
    (t17711, t17712, t17713, t17718, t17720, t17721, t17723, t17724, t17727, t17728, t17729, t17730)
}
