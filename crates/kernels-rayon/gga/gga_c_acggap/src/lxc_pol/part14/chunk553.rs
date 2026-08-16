//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 553/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk553(t1636: f64, t377: f64, t553: f64, t848: f64, t1603: f64, t394: f64, t3457: f64, t406: f64, t1629: f64, t3073: f64, t1647: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4234 = 0.13170898365871023197e1_f64 * t377 * t1636;
    let t4235 = t848 * t553;
    let t4237 = t394 * t1603;
    let t4241 = t3457 * t406;
    let t4242 = t1629 * t4241;
    let t4244 = 0.26341796731742046394e1_f64 * t3073 * t4242;
    let t4245 = t1647 * t864;
    (t4234, t4235, t4237, t4241, t4244, t4245)
}
