//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 346/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk346(t1306: f64, t1309: f64, t469: f64, t465: f64) -> (f64, f64, f64, f64) {
    let t1311 = -t1306 - 0.17808333333333333333e-1_f64 * t1309;
    let t1313 = 0.62182e-1_f64 * t1311 * t469;
    let t1314 = t465 * t465;
    let t1315 = 1.0_f64 / t1314;
    (t1311, t1313, t1314, t1315)
}
