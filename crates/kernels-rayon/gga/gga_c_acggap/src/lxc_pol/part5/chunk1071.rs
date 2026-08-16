//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1071/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1071(t1160: f64, t14575: f64, t1629: f64, t1035: f64, t4225: f64, t864: f64, t3646: f64, t550: f64, t1636: f64, t980: f64, t3378: f64, t4194: f64) -> (f64, f64, f64, f64, f64) {
    let t19032 = t1160 * t1629 * t14575;
    let t19038 = t1035 * t4225 * t864;
    let t19040 = t3646 * t550;
    let t19042 = t980 * t1636;
    let t19045 = t3378 * t4194;
    (t19032, t19038, t19040, t19042, t19045)
}
