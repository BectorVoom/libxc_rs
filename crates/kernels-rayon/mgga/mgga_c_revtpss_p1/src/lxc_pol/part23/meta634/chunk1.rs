//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2331/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2331(t121: f64, t4: f64, t131: f64, t268: f64, t8779: f64, t588: f64, t9282: f64, t239: f64, t2456: f64) -> (f64, f64, f64, f64) {
    let t39484 = t121 * t4;
    let t39490 = 1.0_f64 / t131 / t39484 * t121 * t8779 * t268 / 48.0_f64;
    let t39492 = t9282 * t588;
    let t39494 = t2456 * t239;
    (t39484, t39490, t39492, t39494)
}
