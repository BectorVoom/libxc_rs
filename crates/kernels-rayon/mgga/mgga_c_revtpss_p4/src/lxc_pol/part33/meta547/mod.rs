//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1929;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1930;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta547(t1518: f64, t7683: f64, t1453: f64, t1519: f64, t2322: f64, t27060: f64, t28062: f64, t28065: f64, t28069: f64, t28165: f64, t28170: f64, t28175: f64, t28179: f64, t29427: f64, t29437: f64, t4254: f64, t569: f64, t651: f64, t671: f64, t8158: f64, t8237: f64, t2163: f64, t4292: f64, t670: f64, t8233: f64, t1911: f64, t2165: f64, t28183: f64, t28186: f64, t28188: f64, t28190: f64, t28192: f64, t28193: f64, t28201: f64, t28202: f64, t29432: f64, t4248: f64, t4257: f64, t5787: f64, t7586: f64, t7591: f64, t7687: f64, t29343: f64, t29425: f64, t3: f64, t1461: f64, t1918: f64, t2170: f64, t28257: f64, t28259: f64, t28261: f64, t28263: f64, t28267: f64, t28270: f64, t28273: f64, t28275: f64, t28279: f64, t28282: f64, t573: f64, t5802: f64, t5805: f64, t7696: f64, t8245: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29444, t29451) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1929(t1518, t7683, t1453, t1519, t2322, t27060, t28062, t28065, t28069, t28165, t28170, t28175, t28179, t29427, t29437, t4254, t569, t651, t671, t8158, t8237);
        let (t29456, t29459, t29466) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1930(t2163, t4292, t670, t8233, t1519, t1911, t2165, t28183, t28186, t28188, t28190, t28192, t28193, t28201, t28202, t29432, t4248, t4257, t5787, t651, t7586, t7591, t7687);
        let (t29468, t29469, t29480, t29490) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1931(t29343, t29425, t29451, t29466, t3, t1461, t1918, t2170, t28257, t28259, t28261, t28263, t28267, t28270, t28273, t28275, t28279, t28282, t573, t5802, t5805, t7696, t8245, param_d);
    (t29444, t29456, t29459, t29468, t29469, t29480, t29490)
}
