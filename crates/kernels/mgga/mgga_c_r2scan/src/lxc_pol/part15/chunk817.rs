//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 817/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk817<F: Float>(t8088: F, t8089: F, t2155: F, t2533: F, t2573: F, t5109: F, t2124: F, t2550: F, t7378: F, t1592: F, t2133: F, t6293: F, t6304: F, t6528: F, t8046: F, t8050: F, t8056: F, t8059: F, t8062: F, t8065: F, t8069: F, t8073: F, t8076: F, t8080: F, t8084: F, t8086: F) -> (F, F, F) {
    let t8090 = t8088 * t8089;
    let t8092 = 0.19514881078765566037e-1 * t2155 * t8090;
    let t8093 = t2533 * t2573;
    let t8094 = t5109 * t8093;
    let t8098 = t2124 * t2550 * t7378;
    let t8101 = -t8046 - 0.23115257973478049502e0 * t6304 - 0.2600466522016280569e1 * t6528 * t8050 - t8056 + 0.2600466522016280569e0 * t1592 * t8059 + 0.58544643236296698113e-1 * t8062 - t8065 - t8069 - t8073 + t8076 - t8080 + t8084 + 0.34930954652346593434e-1 * t8086 + t8092 + 0.86682217400542685632e-1 * t2133 * t8094 - 0.16463622957338778997e0 * t6293 * t8098;
    (t8093, t8098, t8101)
}
