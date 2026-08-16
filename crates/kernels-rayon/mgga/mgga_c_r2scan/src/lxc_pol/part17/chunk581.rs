//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 581/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk581(t1577: f64, t1592: f64, t1619: f64, t1622: f64, t2133: f64, t2231: f64, t2651: f64, t2675: f64, t2683: f64, t2685: f64, t2689: f64, t2693: f64, t2696: f64, t2728: f64, t2732: f64, t3064: f64, t3068: f64, t3073: f64, t3077: f64, t3081: f64, t3087: f64, t3092: f64, t3108: f64, t3116: f64, t3227: f64, t535: f64, t574: f64, t948: f64) -> f64 {
    let t3229 = -t1619 - t1622 - 0.27439371595564631661e-1_f64 * t535 * t3064 - 0.43341108700271342816e-1_f64 * t574 * t3068 + 0.54878743191129263322e-1_f64 * t535 * t3073 + 0.86682217400542685632e-1_f64 * t1577 * t3077 - 0.43341108700271342816e-1_f64 * t574 * t3081 - 0.86682217400542685632e-1_f64 * t2651 * t948 - 0.27439371595564631661e-1_f64 * t535 * t3087 + 0.2600466522016280569e0_f64 * t1592 * t3092 + t3108 + 0.25610080155860322884e0_f64 * t2675 - 0.19514881078765566037e-1_f64 * t2683 + 0.54878743191129263322e-2_f64 * t2685 - 0.11643651550782197811e-1_f64 * t2689 - 0.34930954652346593434e-1_f64 * t2693 + 0.86682217400542685632e-1_f64 * t2133 * t3116 - 0.23115257973478049502e0_f64 * t2696 + t2231 + 0.23115257973478049502e0_f64 * t2728 + 0.69345773920434148506e0_f64 * t2732 + t3227;
    t3229
}
