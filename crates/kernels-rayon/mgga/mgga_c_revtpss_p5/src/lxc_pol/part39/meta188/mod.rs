//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk798;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk799;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk800;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta188(t1398: f64, t675: f64, t268: f64, t543: f64, t4101: f64, t1419: f64, t72: f64, t1432: f64, t686: f64, t1433: f64, t2470: f64, t3999: f64, t555: f64, t1385: f64, t1399: f64, t1437: f64, t213: f64, t3924: f64, t4004: f64, t4057: f64, t4066: f64, t4082: f64, t4085: f64, t4090: f64, t4094: f64, t4099: f64, t546: f64, t820: f64, t1427: f64, t1424: f64, t1445: f64, t3894: f64, t3898: f64, t3901: f64, t3904: f64, t3910: f64, t3912: f64, t3918: f64, t3922: f64, t4067: f64, t4071: f64, t4078: f64, t561: f64, t198: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4104, t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk798(t1398, t675, t268, t543, t4101, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let (t4118, t4131) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk799(t1385, t1419, t1399, t1437, t213, t3924, t4004, t4057, t4066, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4114, t546, t820);
        let (t4132, t4135) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk800(t1427, t4131, t1424, t1445, t213, t3894, t3898, t3901, t3904, t3910, t3912, t3918, t3922, t4067, t4071, t4078, t561);
        let t4139 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk801(t198, t531);
    (t4104, t4105, t4107, t4109, t4113, t4114, t4118, t4131, t4132, t4135, t4139)
}
