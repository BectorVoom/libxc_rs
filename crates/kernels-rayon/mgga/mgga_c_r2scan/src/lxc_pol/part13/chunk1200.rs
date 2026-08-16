//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1200/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1200(t3262: f64, t3276: f64, t40397: f64, t37452: f64, t37455: f64, t40360: f64, t40363: f64, t40365: f64, t40368: f64, t40370: f64, t40373: f64, t40377: f64, t40381: f64, t40386: f64, t40388: f64, t40391: f64, t40396: f64) -> (f64, f64) {
    let t40400 = 15.0_f64 / 16.0_f64 * t3262 * t3276 * t40397;
    let t40401 = -t40360 + t40363 + t40365 - t40368 - t37452 - t40370 + t40373 - t40377 - t40381 - 0.38422568777328955684e-2_f64 * t37455 + t40386 - 0.36021158228745895953e-3_f64 * t40388 - 0.72042316457491791906e-3_f64 * t40391 - t40396 + t40400;
    (t40400, t40401)
}
