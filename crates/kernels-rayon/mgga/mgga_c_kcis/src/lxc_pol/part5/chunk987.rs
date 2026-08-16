//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 987/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk987(t12062: f64, t333: f64, t3754: f64, t740: f64, t113: f64, t11425: f64, t11966: f64, t518: f64, t1405: f64, t1441: f64, t1420: f64, t4016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12064 = 0.72818958333333333333e-4_f64 * t333 * t12062;
    let t12065 = t740 * t3754;
    let t12070 = t113 * t11425;
    let t12084 = 0.14055920378328537299e-1_f64 * t11966 * t518;
    let t12085 = t1441 * t1405;
    let t12087 = t4016 * t1420;
    (t12064, t12065, t12070, t12084, t12085, t12087)
}
