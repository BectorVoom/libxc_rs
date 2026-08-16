//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 788/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk788(t9030: f64, t9031: f64, t1027: f64, t1787: f64, t1740: f64, t9016: f64, t9020: f64, t19: f64, t424: f64, t3114: f64, t3117: f64, t3123: f64, t8888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9032 = t9030 * t9031;
    let t9034 = t1027 * t1787;
    let t9036 = t9016 * t1740;
    let t9038 = t9020 * t1740;
    let t9040 = t424 * t19;
    let t9041 = t9040 * t3114;
    let t9042 = t9041 * t3117;
    let t9044 = t8888 * t3123;
    (t9032, t9034, t9036, t9038, t9041, t9042, t9044)
}
