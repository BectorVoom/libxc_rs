//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 549/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk549(t1160: f64, t4163: f64, t441: f64, t524: f64, t1004: f64, t1648: f64, t1529: f64, t310: f64, t1633: f64, t157: f64, t864: f64, t1629: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4164 = t1160 * t4163;
    let t4166 = t441 * t524;
    let t4170 = t1004 * t1648;
    let t4180 = t310 * t1529;
    let t4182 = 0.26341796731742046394e1_f64 * t4180 * t1633;
    let t4183 = t157 * t864;
    let t4184 = t1629 * t4183;
    (t4164, t4166, t4170, t4180, t4182, t4183, t4184)
}
