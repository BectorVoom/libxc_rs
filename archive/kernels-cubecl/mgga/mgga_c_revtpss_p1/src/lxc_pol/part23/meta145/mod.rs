//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta145 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk917;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk918;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk919;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk920;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk921;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta145<F: Float>(t4101: F, t4104: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F, t3999: F, t555: F, t1385: F, t198: F, t531: F, t1448: F, t1450: F, t565: F, t2219: F, t2223: F, t2226: F, t2230: F, t2233: F, t2239: F, t1466: F, t602: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk917::<F>(t4101, t4104, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let t4118 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk918::<F>(t1385, t1419);
        let t4139 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk919::<F>(t198, t531);
        let t4140 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk920::<F>(t1448, t1450);
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk921::<F>(t565);
        let (t4171, t4173) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk922::<F>(t2219, t2223, t2226, t2230, t2233, t2239, t1466, t602);
    (t4105, t4107, t4109, t4113, t4114, t4118, t4139, t4140, t4146, t4147, t4171, t4173)
}
