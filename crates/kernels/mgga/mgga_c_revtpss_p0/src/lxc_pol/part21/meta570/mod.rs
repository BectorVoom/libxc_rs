//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2270;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2271;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2272;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta570<F: Float>(t13045: F, t3601: F, t17710: F, t3720: F, t1261: F, t12784: F, t17669: F, t17674: F, t17679: F, t17684: F, t17690: F, t17693: F, t17696: F, t17700: F, t17705: F, t17709: F, t3625: F, t3708: F, t5287: F, t5331: F, t5340: F, t5407: F, t3172: F, t5303: F, t17633: F, t5352: F, t1209: F, t489: F, t3623: F, t370: F, t1214: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17711, t17712, t17713, t17718) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2270::<F>(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
        let (t17720, t17721, t17723, t17724, t17727, t17728) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2271::<F>(t3172, t5303, t1261, t17633, t5352, t3720, t1209, t489, t3623, t370);
        let t17729 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2272::<F>(t17727, t17728);
        let t17730 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2273::<F>(t1214, t606);
    (t17711, t17712, t17713, t17718, t17720, t17721, t17723, t17724, t17728, t17729, t17730)
}
