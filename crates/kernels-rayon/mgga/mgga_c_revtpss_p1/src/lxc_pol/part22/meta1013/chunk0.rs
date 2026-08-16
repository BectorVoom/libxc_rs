//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3479/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3479(t19894: f64, t3127: f64, t3172: f64, t15707: f64, t15734: f64, t19882: f64, t3188: f64, t16190: f64, t4820: f64, t1063: f64, t19662: f64, t19667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65444 = t3127 * t3172 * t19894;
    let t65446 = t15707 * t15734;
    let t65454 = t3188 * t19882;
    let t65456 = t16190 * t4820;
    let t65459 = t1063 * t3172 * t19662;
    let t65462 = t1063 * t3172 * t19667;
    (t65444, t65446, t65454, t65456, t65459, t65462)
}
