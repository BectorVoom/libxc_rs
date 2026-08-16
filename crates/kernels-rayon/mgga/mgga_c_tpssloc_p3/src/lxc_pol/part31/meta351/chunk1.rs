//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1263/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1263(t135: f64, t4930: f64, t1174: f64, t1420: f64, t1887: f64, t337: f64, t11570: f64, t3961: f64, t1714: f64, t4899: f64, t11545: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t15372 = t135 * t4930;
    let t15374 = 0.55555555555555555554e-3_f64 * t1174 * t15372;
    let t15376 = t1420 * t337 * t1887;
    let t15382 = t11570 * t3961;
    let t15390 = t4899 * t1714;
    let t15394 = t60 * t11545;
    (t15374, t15376, t15382, t15390, t15394)
}
