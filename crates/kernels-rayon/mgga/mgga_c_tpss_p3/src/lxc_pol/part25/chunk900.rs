//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 900/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk900(t2549: f64, t872: f64, t2593: f64, t891: f64, t2618: f64, t309: f64, t8772: f64, t8749: f64, t8660: f64, t650: f64, t969: f64, t242: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8899 = t872 * t2549;
    let t8906 = t891 * t2593;
    let t8912 = t891 * t2618;
    let t8915 = t309 * t8772;
    let t8922 = t309 * t8749;
    let t8927 = 0.53272592592592592592e-1_f64 * t8660;
    let t8951 = t650 * t969;
    let t8953 = t242 * t8951 * t837;
    (t8899, t8906, t8912, t8915, t8922, t8927, t8951, t8953)
}
