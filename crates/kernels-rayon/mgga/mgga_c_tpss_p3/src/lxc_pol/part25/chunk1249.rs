//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1249/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1249(t17964: f64, t4724: f64, t4761: f64, t5552: f64, t4766: f64, t4771: f64, t5559: f64, t4775: f64, t1705: f64, t4778: f64, t935: f64, t1378: f64, t1395: f64, t226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21280 = t17964 * t4724;
    let t21282 = t5552 * t4761;
    let t21284 = t5552 * t4766;
    let t21286 = t5559 * t4771;
    let t21288 = t5559 * t4775;
    let t21298 = t1705 * t4778;
    let t21299 = t21298 * t935;
    let t21312 = t1395 * t1378 * t226;
    (t21280, t21282, t21284, t21286, t21288, t21298, t21299, t21312)
}
