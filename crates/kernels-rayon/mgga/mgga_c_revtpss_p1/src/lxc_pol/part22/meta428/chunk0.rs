//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2049/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2049(t1531: f64, t37: f64, t2612: f64, t4392: f64, t72: f64, t757: f64, t14425: f64, t150: f64, t190: f64, t10608: f64, t2258: f64, t4402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14613 = t37 * t1531;
    let t14615 = 12.0_f64 * t14613 * t2612;
    let t14616 = t4392 * t72;
    let t14618 = 0.36622894612013090108e-3_f64 * t14616 * t757;
    let t14619 = t150 * t14425;
    let t14620 = t14619 * t190;
    let t14621 = 0.23392894490538584828e1_f64 * t10608;
    let t14622 = t4402 * t2258;
    (t14613, t14615, t14616, t14618, t14619, t14620, t14621, t14622)
}
