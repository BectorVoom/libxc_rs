//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 816/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk816(t3356: f64, t1156: f64, t1160: f64, t1159: f64, t431: f64) -> (f64, f64, f64, f64) {
    let t3439 = 0.22831111111111111111e-1_f64 * t3356;
    let t3447 = t1156 * t1160;
    let t3450 = t1159 * t431;
    let t3451 = 1.0_f64 / t3450;
    (t3439, t3447, t3450, t3451)
}
