//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 726/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk726(t1167: f64, t7647: f64, t1103: f64, t1998: f64, t1108: f64, t1113: f64, t137: f64, t922: f64, t1095: f64, t4352: f64, t598: f64, t1086: f64, t2001: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7648 = t7647 * t1167;
    let t7649 = 0.85748036236139473944e-3_f64 * t7648;
    let t7650 = t1998 * t1103;
    let t7651 = 0.34299214494455789578e-2_f64 * t7650;
    let t7652 = t1998 * t1108;
    let t7653 = 0.17149607247227894789e-2_f64 * t7652;
    let t7654 = t1998 * t1113;
    let t7655 = 0.17149607247227894789e-2_f64 * t7654;
    let t7656 = t137 * t922;
    let t7658 = t4352 * t1095 * t7656;
    let t7659 = t598 * t7658;
    let t7661 = t2001 * t1086;
    (t7648, t7649, t7650, t7651, t7652, t7653, t7654, t7655, t7656, t7658, t7659, t7661)
}
