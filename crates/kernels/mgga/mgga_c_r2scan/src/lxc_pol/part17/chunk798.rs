//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 798/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk798<F: Float>(t574: F, t9226: F, t2620: F, t2651: F, t1632: F, t3090: F, t551: F, t1592: F, t133: F, t2892: F, t6243: F, t1604: F, t1584: F, t2196: F, t3081: F, t3092: F, t6425: F, t6528: F, t7500: F, t7566: F, t9190: F, t9194: F, t9202: F, t9209: F, t9214: F, t9219: F, t9221: F, t9223: F, t948: F) -> (F, F, F) {
    let t9227 = t574 * t9226;
    let t9229 = t2651 * t2620;
    let t9232 = t551 * t1632 * t3090;
    let t9233 = t1592 * t9232;
    let t9235 = t133 * t2892;
    let t9236 = t6243 * t9235;
    let t9237 = t1604 * t9236;
    let t9239 = -0.2600466522016280569e1 * t6528 * t9190 + 0.10401866088065122276e1 * t2196 * t9194 - 0.86682217400542685632e-1 * t7566 * t948 - 0.43341108700271342816e-1 * t1584 * t3081 - 0.43341108700271342816e-1 * t574 * t9202 + 0.2600466522016280569e0 * t6425 * t3092 + 0.2600466522016280569e0 * t1592 * t9209 + 0.2600466522016280569e0 * t1592 * t9214 + 0.11557628986739024751e0 * t9219 + 0.64025200389650807209e-1 * t9221 + 0.64025200389650807209e-1 * t9223 + 0.11557628986739024751e0 * t9227 + 0.23115257973478049502e0 * t9229 - 0.69345773920434148507e0 * t9233 - t7500 - 0.16463622957338778997e-1 * t9237;
    (t9235, t9236, t9239)
}
