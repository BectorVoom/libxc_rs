//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 869/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk869<F: Float>(t6243: F, t9235: F, t1604: F, t1584: F, t1592: F, t2196: F, t3081: F, t3092: F, t574: F, t6425: F, t6528: F, t7500: F, t7566: F, t9190: F, t9194: F, t9202: F, t9209: F, t9214: F, t9219: F, t9221: F, t9223: F, t9227: F, t9229: F, t9233: F, t948: F) -> (F, F) {
    let t9236 = t6243 * t9235;
    let t9237 = t1604 * t9236;
    let t9239 = -F::cast_from(0.2600466522016280569e1_f64) * t6528 * t9190 + F::cast_from(0.10401866088065122276e1_f64) * t2196 * t9194 - F::cast_from(0.86682217400542685632e-1_f64) * t7566 * t948 - F::cast_from(0.43341108700271342816e-1_f64) * t1584 * t3081 - F::cast_from(0.43341108700271342816e-1_f64) * t574 * t9202 + F::cast_from(0.2600466522016280569e0_f64) * t6425 * t3092 + F::cast_from(0.2600466522016280569e0_f64) * t1592 * t9209 + F::cast_from(0.2600466522016280569e0_f64) * t1592 * t9214 + F::cast_from(0.11557628986739024751e0_f64) * t9219 + F::cast_from(0.64025200389650807209e-1_f64) * t9221 + F::cast_from(0.64025200389650807209e-1_f64) * t9223 + F::cast_from(0.11557628986739024751e0_f64) * t9227 + F::cast_from(0.23115257973478049502e0_f64) * t9229 - F::cast_from(0.69345773920434148507e0_f64) * t9233 - t7500 - F::cast_from(0.16463622957338778997e-1_f64) * t9237;
    (t9236, t9239)
}
