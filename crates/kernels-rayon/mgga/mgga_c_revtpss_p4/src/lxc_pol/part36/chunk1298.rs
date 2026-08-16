//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1298/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1298(t30993: f64, t571: f64, t2167: f64, t6951: f64, t1913: f64, t8249: f64, t29508: f64, t7742: f64, t29502: f64, t7732: f64, t30123: f64, t98450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113025 = t571 * t30993;
    let t113053 = t2167 * t6951;
    let t113054 = t1913 * t8249;
    let t113063 = 6.0_f64 * t29508 * t7742;
    let t113065 = 12.0_f64 * t7732 * t29502;
    let t113067 = 18.0_f64 * t98450 * t30123;
    (t113025, t113053, t113054, t113063, t113065, t113067)
}
