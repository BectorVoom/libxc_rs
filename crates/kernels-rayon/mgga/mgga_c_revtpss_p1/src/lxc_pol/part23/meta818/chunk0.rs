//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2665/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665(t19676: f64, t3127: f64, t3172: f64, t16158: f64, t4834: f64, t19791: f64, t19781: f64, t3091: f64, t43131: f64, t19939: f64, t11262: f64, t3161: f64, t6311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65527 = t3127 * t3172 * t19676;
    let t65538 = t4834 * t16158;
    let t65553 = t3127 * t3172 * t19791;
    let t65567 = t3091 * t43131 * t19781;
    let t65570 = t3127 * t3172 * t19939;
    let t65581 = t3161 * t11262 * t6311;
    (t65527, t65538, t65553, t65567, t65570, t65581)
}
