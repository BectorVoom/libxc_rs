//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 869/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk869(t6243: f64, t9235: f64, t1604: f64, t1584: f64, t1592: f64, t2196: f64, t3081: f64, t3092: f64, t574: f64, t6425: f64, t6528: f64, t7500: f64, t7566: f64, t9190: f64, t9194: f64, t9202: f64, t9209: f64, t9214: f64, t9219: f64, t9221: f64, t9223: f64, t9227: f64, t9229: f64, t9233: f64, t948: f64) -> (f64, f64) {
    let t9236 = t6243 * t9235;
    let t9237 = t1604 * t9236;
    let t9239 = -0.2600466522016280569e1_f64 * t6528 * t9190 + 0.10401866088065122276e1_f64 * t2196 * t9194 - 0.86682217400542685632e-1_f64 * t7566 * t948 - 0.43341108700271342816e-1_f64 * t1584 * t3081 - 0.43341108700271342816e-1_f64 * t574 * t9202 + 0.2600466522016280569e0_f64 * t6425 * t3092 + 0.2600466522016280569e0_f64 * t1592 * t9209 + 0.2600466522016280569e0_f64 * t1592 * t9214 + 0.11557628986739024751e0_f64 * t9219 + 0.64025200389650807209e-1_f64 * t9221 + 0.64025200389650807209e-1_f64 * t9223 + 0.11557628986739024751e0_f64 * t9227 + 0.23115257973478049502e0_f64 * t9229 - 0.69345773920434148507e0_f64 * t9233 - t7500 - 0.16463622957338778997e-1_f64 * t9237;
    (t9236, t9239)
}
