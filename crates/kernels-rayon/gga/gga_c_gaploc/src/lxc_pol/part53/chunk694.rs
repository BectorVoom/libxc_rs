//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 694/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk694(t2877: f64, t3153: f64, t2854: f64, t3116: f64, t1445: f64, t1562: f64, t1645: f64, t3133: f64, t8352: f64, t2778: f64, t574: f64, t12446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12911 = 0.35750489951850426669e0_f64 * t3153 * t2877;
    let t12918 = t2854 * t3116;
    let t12919 = t1445 * t12918;
    let t12921 = 0.69017266717057349418e1_f64 * t1562 * t12919;
    let t12922 = t1645 * t3133;
    let t12924 = 0.42900587942220512003e1_f64 * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = 0.46011511144704899612e1_f64 * t574 * t12926;
    let t12929 = 0.63904876589867916127e-1_f64 * t12446;
    (t12911, t12918, t12919, t12921, t12922, t12924, t12925, t12926, t12928, t12929)
}
