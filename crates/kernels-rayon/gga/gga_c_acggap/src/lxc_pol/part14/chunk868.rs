//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 868/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk868(t1985: f64, t30196: f64, t3668: f64, t587: f64, t381: f64, t390: f64, t151: f64) -> (f64, f64, f64) {
    let t30242 = t30196 * t1985;
    let t30243 = 0.21437009059034868486e-3_f64 * t30242;
    let t30244 = t587 * t3668;
    let t30246 = t381 * t30244 * t390;
    let t30247 = 0.34013387707001991332e-1_f64 * t30246;
    let t30248 = t151 * t30244;
    (t30243, t30247, t30248)
}
