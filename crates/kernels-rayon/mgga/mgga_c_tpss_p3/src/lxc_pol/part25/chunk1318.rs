//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1318/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1318(t4701: f64, t580: f64, t14029: f64, t30: f64, t21298: f64, t5570: f64, t14322: f64, t17964: f64, t14326: f64, t14343: f64, t19703: f64, t14189: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69887 = t580 * t4701;
    let t69891 = t30 * t14029;
    let t69912 = t21298 * t5570;
    let t69926 = t17964 * t14322;
    let t69928 = t17964 * t14326;
    let t69930 = t19703 * t14343;
    let t69932 = t17964 * t14189;
    (t69887, t69891, t69912, t69926, t69928, t69930, t69932)
}
