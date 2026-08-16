//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 353/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk353(t44: f64, t51: f64, t406: f64, t425: f64, t458: f64, t99: f64, t1213: f64, t1219: f64, t48: f64, t101: f64, t1225: f64, t1228: f64, t53: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t1357 = t406 * t425;
    let t1358 = 8.0_f64 * t1357;
    let t1359 = t406 * t458;
    let t1360 = 8.0_f64 * t1359;
    let t1361 = 1.0_f64 / t99;
    let t1367 = piecewise3(t45, 0.0_f64, 4.0_f64 / 9.0_f64 * t1361 * t1213 + 4.0_f64 / 3.0_f64 * t48 * t1219);
    let t1368 = 1.0_f64 / t101;
    let t1374 = piecewise3(t52, 0.0_f64, 4.0_f64 / 9.0_f64 * t1368 * t1225 + 4.0_f64 / 3.0_f64 * t53 * t1228);
    let t1375 = t1367 + t1374;
    (t1357, t1358, t1360, t1361, t1368, t1375)
}
