//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta145 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk917;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk918;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk919;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk920;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk921;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta145(t4101: f64, t4104: f64, t1419: f64, t72: f64, t1432: f64, t686: f64, t1433: f64, t2470: f64, t3999: f64, t555: f64, t1385: f64, t198: f64, t531: f64, t1448: f64, t1450: f64, t565: f64, t2219: f64, t2223: f64, t2226: f64, t2230: f64, t2233: f64, t2239: f64, t1466: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk917(t4101, t4104, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let t4118 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk918(t1385, t1419);
        let t4139 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk919(t198, t531);
        let t4140 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk920(t1448, t1450);
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk921(t565);
        let (t4171, t4173) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk922(t2219, t2223, t2226, t2230, t2233, t2239, t1466, t602);
    (t4105, t4107, t4109, t4113, t4114, t4118, t4139, t4140, t4146, t4147, t4171, t4173)
}
