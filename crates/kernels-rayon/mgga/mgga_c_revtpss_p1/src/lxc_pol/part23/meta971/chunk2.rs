//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3281/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3281(t22822: f64, t3989: f64, t2661: f64, t3992: f64, t543: f64, t86205: f64, t1353: f64, t1410: f64, t1414: f64, t221: f64, t22852: f64, t49071: f64, t49093: f64, t74638: f64, t74641: f64, t74656: f64, t74660: f64, t74664: f64, t828: f64, t85442: f64, t86203: f64, t86208: f64, t86212: f64, t86220: f64, t86222: f64, t86226: f64, t86234: f64, t86236: f64) -> f64 {
    let t86240 = t3989 * t22822;
    let t86244 = t2661 * t3992 * t86205 * t543;
    let t86249 = 0.71456696863449561619e-5_f64 * t86203 + 0.42874018118069736973e-4_f64 * t86208 - 0.42874018118069736973e-4_f64 * t86212 - 0.85748036236139473944e-3_f64 * t1410 * t1414 * t828 * t85442 - 0.50820002809285328225e-4_f64 * t86220 + 0.12004725073059526352e0_f64 * t86222 - 0.15246000842785598467e-2_f64 * t86226 - t49071 - 3.0_f64 / 4.0_f64 * t49093 * t221 * t22852 * t1353 + 0.21437009059034868486e-4_f64 * t86234 + 7.0_f64 / 12.0_f64 * t86236 - 0.6098400337114239387e-4_f64 * t74638 - 0.13553694749236397037e-4_f64 * t74641 + 0.40015750243531754507e-2_f64 * t86240 + 0.71456696863449561619e-5_f64 * t86244 - 0.24009450146119052704e-1_f64 * t74656 - 0.30492001685571196935e-3_f64 * t74660 + 0.15246000842785598467e-3_f64 * t74664;
    t86249
}
