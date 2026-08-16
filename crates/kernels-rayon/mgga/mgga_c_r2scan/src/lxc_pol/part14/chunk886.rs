//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 886/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk886(t8088: f64, t8089: f64, t2155: f64, t2533: f64, t2573: f64, t5109: f64, t2124: f64, t2550: f64, t7378: f64, t1592: f64, t2133: f64, t6293: f64, t6304: f64, t6528: f64, t8046: f64, t8050: f64, t8056: f64, t8059: f64, t8062: f64, t8065: f64, t8069: f64, t8073: f64, t8076: f64, t8080: f64, t8084: f64, t8086: f64) -> (f64, f64, f64) {
    let t8090 = t8088 * t8089;
    let t8092 = 0.19514881078765566037e-1_f64 * t2155 * t8090;
    let t8093 = t2533 * t2573;
    let t8094 = t5109 * t8093;
    let t8098 = t2124 * t2550 * t7378;
    let t8101 = -t8046 - 0.23115257973478049502e0_f64 * t6304 - 0.2600466522016280569e1_f64 * t6528 * t8050 - t8056 + 0.2600466522016280569e0_f64 * t1592 * t8059 + 0.58544643236296698113e-1_f64 * t8062 - t8065 - t8069 - t8073 + t8076 - t8080 + t8084 + 0.34930954652346593434e-1_f64 * t8086 + t8092 + 0.86682217400542685632e-1_f64 * t2133 * t8094 - 0.16463622957338778997e0_f64 * t6293 * t8098;
    (t8093, t8098, t8101)
}
