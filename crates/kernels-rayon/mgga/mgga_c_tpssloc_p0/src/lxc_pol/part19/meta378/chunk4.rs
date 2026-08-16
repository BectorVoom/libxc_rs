//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1415/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415(t11292: f64, t1156: f64, t1164: f64, t43679: f64, t43748: f64, t43750: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64) -> (f64, f64) {
    let t43924 = 0.14035736694323150897e2_f64 * t1164 * t11292 * t43679 * t1156;
    let t43936 = -0.16481481481481481482e-1_f64 * t43748 - 0.13734567901234567901e-1_f64 * t43750 + 0.24722222222222222222e-1_f64 * t43780 + 0.49444444444444444445e-1_f64 * t43782 + 0.49444444444444444444e-1_f64 * t43784 - 0.74166666666666666668e-1_f64 * t43786 - 0.12361111111111111111e-1_f64 * t43788 + 0.12361111111111111111e0_f64 * t43794 - 0.22249999999999999999e0_f64 * t43798 + 0.2225e0_f64 * t43802 + 0.92708333333333333333e-2_f64 * t43806;
    (t43924, t43936)
}
