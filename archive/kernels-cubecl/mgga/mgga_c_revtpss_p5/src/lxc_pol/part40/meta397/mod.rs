//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1443;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1444;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta397<F: Float>(t15904: F, t3623: F, t13148: F, t11249: F, t1794: F, t13045: F, t3601: F, t3720: F, t1261: F, t12784: F, t17669: F, t17674: F, t17679: F, t17684: F, t17690: F, t17693: F, t17696: F, t17700: F, t17705: F, t3625: F, t3708: F, t5287: F, t5331: F, t5340: F, t5407: F, t3172: F, t5303: F, t17633: F, t5352: F, t1209: F, t489: F, t370: F, t1214: F, t606: F, t5051: F, t3626: F, t3566: F, t1121: F, t1774: F, t3584: F, t471: F, t5351: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17708, t17709, t17710) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1442::<F>(t15904, t3623, t13148, t11249, t1794);
        let t17718 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1443::<F>(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
        let (t17721, t17724, t17728, t17729, t17730) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1444::<F>(t3172, t5303, t1261, t17633, t5352, t3720, t1209, t489, t3623, t370, t1214, t606);
        let (t17732, t17736, t17739, t17744) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1445::<F>(t17730, t5051, t3626, t3566, t489, t17728, t1121, t1774, t3584, t471, t5351, t3720);
    (t17708, t17710, t17718, t17721, t17724, t17729, t17730, t17732, t17736, t17739, t17744)
}
