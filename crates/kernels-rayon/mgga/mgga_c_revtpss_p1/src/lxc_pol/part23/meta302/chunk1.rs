//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1561/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1561(t3434: f64, t421: f64, t1126: f64, t3432: f64, t3431: f64, t418: f64, t408: f64) -> (f64, f64, f64, f64) {
    let t12230 = 1.0_f64 / t3434 / t421;
    let t12243 = t1126 * t3432;
    let t12247 = 1.0_f64 / t3431 / t418;
    let t12248 = t408 * t12247;
    (t12230, t12243, t12247, t12248)
}
