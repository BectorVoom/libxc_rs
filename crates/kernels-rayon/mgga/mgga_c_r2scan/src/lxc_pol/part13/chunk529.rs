//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 529/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk529(t2483: f64, t88: f64, t41: f64, t410: f64, t899: f64, t1388: f64, t1356: f64, t1387: f64, t1413: f64, t1418: f64, t1421: f64, t1511: f64, t2451: f64, t2453: f64, t2455: f64, t2465: f64) -> (f64, f64, f64, f64, f64) {
    let t2484 = t2483 * t88;
    let t2485 = t41 * t2484;
    let t2486 = t410 * t899;
    let t2487 = 4.0_f64 * t2486;
    let t2488 = 0.5848223622634646207e0_f64 * t1388;
    let t2489 = -t1356 - t2451 + t2453 + t2455 - t2465 + t2485 - t2487 - t1387 - t2488 - t1413 + t1418 - t1421 + t1511;
    (t2484, t2485, t2487, t2488, t2489)
}
