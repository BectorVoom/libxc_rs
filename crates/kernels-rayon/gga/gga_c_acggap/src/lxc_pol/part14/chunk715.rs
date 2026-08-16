//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 715/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk715(t1167: f64, t7647: f64, t1103: f64, t1998: f64, t1108: f64, t1113: f64, t1089: f64, t368: f64, t7554: f64, t7553: f64, t2037: f64, t7309: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7648 = t7647 * t1167;
    let t7650 = t1998 * t1103;
    let t7652 = t1998 * t1108;
    let t7654 = t1998 * t1113;
    let t7670 = t1089 * t368 * t7554;
    let t7671 = t7553 * t7670;
    let t7672 = 0.21437009059034868486e-3_f64 * t7671;
    let t7673 = t7309 * t2037;
    (t7648, t7650, t7652, t7654, t7670, t7672, t7673)
}
