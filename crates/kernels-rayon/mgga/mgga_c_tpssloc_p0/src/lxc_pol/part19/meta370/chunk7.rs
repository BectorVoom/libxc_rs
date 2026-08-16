//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1378/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1378(t11126: f64, t3423: f64, t11286: f64, t3411: f64, t11629: f64, t11399: f64, t1164: f64, t3400: f64, t4883: f64, t3377: f64) -> (f64, f64, f64, f64, f64) {
    let t43670 = 0.10389515463408878255e3_f64 * t11126 * t3423;
    let t43672 = 0.4101607543286562663e4_f64 * t3411 * t11286;
    let t43674 = 0.14035736694323150897e2_f64 * t3411 * t11629;
    let t43678 = 0.69263436422725855036e2_f64 * t1164 * t3400 * t11399 * t4883;
    let t43679 = t3377 * t3377;
    (t43670, t43672, t43674, t43678, t43679)
}
