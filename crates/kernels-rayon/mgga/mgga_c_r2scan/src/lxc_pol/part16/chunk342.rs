//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 342/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk342(t406: f64, t425: f64, t458: f64, t99: f64, t101: f64, t445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1357 = t406 * t425;
    let t1359 = t406 * t458;
    let t1360 = 8.0_f64 * t1359;
    let t1361 = 1.0_f64 / t99;
    let t1368 = 1.0_f64 / t101;
    let t1379 = t445 * t445;
    let t1380 = 1.0_f64 / t1379;
    (t1357, t1360, t1361, t1368, t1379, t1380)
}
