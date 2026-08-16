//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk839;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk840;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk841;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk842;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk843;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk844;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk845;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta155<F: Float>(t1398: F, t675: F, t268: F, t543: F, t4101: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F, t3999: F, t555: F, t1385: F, t1399: F, t1437: F, t213: F, t3924: F, t4004: F, t4057: F, t4066: F, t4082: F, t4085: F, t4090: F, t4094: F, t4099: F, t546: F, t820: F, t1427: F, t1424: F, t1445: F, t3894: F, t3898: F, t3901: F, t3904: F, t3910: F, t3912: F, t3918: F, t3922: F, t4067: F, t4071: F, t4078: F, t561: F, t198: F, t531: F, t1448: F, t1450: F, t565: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4104, t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk839::<F>(t1398, t675, t268, t543, t4101, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let (t4118, t4131) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk840::<F>(t1385, t1419, t1399, t1437, t213, t3924, t4004, t4057, t4066, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4114, t546, t820);
        let t4132 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk841::<F>(t1427, t4131);
        let t4135 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk842::<F>(t1424, t1445, t213, t3894, t3898, t3901, t3904, t3910, t3912, t3918, t3922, t4067, t4071, t4078, t4132, t561);
        let t4139 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk843::<F>(t198, t531);
        let t4140 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk844::<F>(t1448, t1450);
        let t4144 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk845::<F>(t1448);
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk846::<F>(t565);
    (t4104, t4107, t4114, t4118, t4131, t4132, t4135, t4139, t4140, t4144, t4146, t4147)
}
