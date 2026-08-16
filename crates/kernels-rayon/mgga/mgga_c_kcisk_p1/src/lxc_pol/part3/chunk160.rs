//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 160/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk160(t571: f64, t574: f64, t577: f64, t581: f64, t591: f64, t45: f64, t589: f64) -> (f64, f64, f64, f64, f64) {
    let t596 = 0.51785e1_f64 * t574 + 0.905775e0_f64 * t571 + 0.1100325e0_f64 * t577 + 0.1241775e0_f64 * t581;
    let t599 = 1.0_f64 + 0.29608574643216675549e2_f64 / t596;
    let t600 = f64::ln(t599);
    let t601 = t591 * t600;
    let t604 = -t589 + 0.19751789702565206229e-1_f64 * t45 * t601;
    (t596, t599, t600, t601, t604)
}
