//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 967/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk967(t15659: f64, t19056: f64, t4582: f64, t1735: f64, t1653: f64, t6225: f64, t3578: f64, t6230: f64, t5975: f64, t1734: f64, t6224: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22270 = t19056 * t15659;
    let t22271 = t4582 * t22270;
    let t22274 = t19056 * t1735;
    let t22275 = t4582 * t22274;
    let t22279 = t6225 * t1653;
    let t22280 = t3578 * t22279;
    let t22283 = t6230 * t1653;
    let t22284 = t3578 * t22283;
    let t22287 = t1735 * t5975;
    let t22288 = t3578 * t22287;
    let t22298 = t6224 * t1734;
    let t22299 = t22298 * t475;
    (t22271, t22275, t22280, t22284, t22288, t22298, t22299)
}
