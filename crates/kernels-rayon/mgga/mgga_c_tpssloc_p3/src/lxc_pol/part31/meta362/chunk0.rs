//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1283/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1283(t1354: f64, t16288: f64, t12211: f64, t5223: f64, t3804: f64, t820: f64, t1351: f64, t1824: f64, t3792: f64, t12345: f64, t1831: f64, t1362: f64, t16060: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16290 = 7.0_f64 / 2304.0_f64 * t16288 * t1354;
    let t16294 = 7.0_f64 / 24.0_f64 * t12211 * t5223;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    let t16321 = t16060 * t1362;
    (t16290, t16294, t16305, t16306, t16311, t16317, t16321)
}
