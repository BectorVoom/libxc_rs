//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 937/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk937(t1562: f64, t8817: f64, t31176: f64, t681: f64, t5016: f64, t9765: f64, t2310: f64, t38638: f64, t16156: f64, t9975: f64, t8616: f64, t8676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45591 = t1562 * t8817;
    let t45593 = t31176 * t681;
    let t45595 = t5016 * t9765;
    let t45597 = t38638 * t2310;
    let t45599 = t16156 * t9975;
    let t45601 = t8676 * t8616;
    (t45591, t45593, t45595, t45597, t45599, t45601)
}
