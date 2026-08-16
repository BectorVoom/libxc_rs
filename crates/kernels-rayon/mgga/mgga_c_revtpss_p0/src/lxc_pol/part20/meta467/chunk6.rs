//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1792/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1792(t4144: f64, t4146: f64, t198: f64, t25177: f64, t39989: f64, t4135: f64, t4139: f64, t4140: f64, t47076: f64, t47079: f64, t47082: f64, t47084: f64, t47086: f64, t47088: f64, t47090: f64, t47092: f64, t47094: f64, t47096: f64, t47098: f64, t532: f64, t5541: f64, t9628: f64) -> f64 {
    let t47669 = t4144 * t4144;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t47676 = -6.0_f64 * t198 * t47669 * t47672 * t532 + 12.0_f64 * t25177 * t4135 * t5541 + 12.0_f64 * t4139 * t4140 * t9628 - t39989 - t47076 - t47079 + t47082 - t47084 - t47086 + t47088 + t47090 + t47092 + t47094 - t47096 - t47098;
    t47676
}
