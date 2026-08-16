//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1443;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1444;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta397(t15904: f64, t3623: f64, t13148: f64, t11249: f64, t1794: f64, t13045: f64, t3601: f64, t3720: f64, t1261: f64, t12784: f64, t17669: f64, t17674: f64, t17679: f64, t17684: f64, t17690: f64, t17693: f64, t17696: f64, t17700: f64, t17705: f64, t3625: f64, t3708: f64, t5287: f64, t5331: f64, t5340: f64, t5407: f64, t3172: f64, t5303: f64, t17633: f64, t5352: f64, t1209: f64, t489: f64, t370: f64, t1214: f64, t606: f64, t5051: f64, t3626: f64, t3566: f64, t1121: f64, t1774: f64, t3584: f64, t471: f64, t5351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17708, t17709, t17710) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1442(t15904, t3623, t13148, t11249, t1794);
        let t17718 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1443(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
        let (t17721, t17724, t17728, t17729, t17730) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1444(t3172, t5303, t1261, t17633, t5352, t3720, t1209, t489, t3623, t370, t1214, t606);
        let (t17732, t17736, t17739, t17744) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1445(t17730, t5051, t3626, t3566, t489, t17728, t1121, t1774, t3584, t471, t5351, t3720);
    (t17708, t17710, t17718, t17721, t17724, t17729, t17730, t17732, t17736, t17739, t17744)
}
