//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 572/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk572(t1356: f64, t1387: f64, t1413: f64, t1418: f64, t2045: f64, t2052: f64, t2059: f64, t2063: f64, t2451: f64, t2453: f64, t2455: f64, t2465: f64, t2485: f64, t2487: f64, t2488: f64, t2810: f64, t2813: f64, t2816: f64, t765: f64) -> f64 {
    let t2819 = 0.285764e-1_f64 * t2045 + t2052 - t2059 - 0.675260332e-1_f64 * t2063 + t1356 + t2451 - t2453 - t2455 + t2465 - t2485 + 0.675260332e-1_f64 * t765 * t2810 + 0.675260332e-1_f64 * t765 * t2813 + 0.675260332e-1_f64 * t765 * t2816 + t2487 + t1387 + t2488 + t1413 - t1418;
    t2819
}
