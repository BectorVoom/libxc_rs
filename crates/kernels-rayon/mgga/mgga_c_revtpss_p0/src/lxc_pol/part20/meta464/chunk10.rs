//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1774/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1774(t10008: f64, t1432: f64, t686: f64, t72: f64, t268: f64, t39644: f64, t546: f64, t555: f64, t8779: f64, t4107: f64, t9288: f64, t10107: f64, t3964: f64, t9285: f64) -> (f64, f64, f64, f64) {
    let t47436 = t1432 * t10008 * t72 * t686;
    let t47442 = 0.11638313500518478545e-4_f64 * t39644 * t546 * t555 * t8779 * t268;
    let t47444 = t1432 * t4107 * t9288;
    let t47450 = t3964 * t10107 * t9285;
    (t47436, t47442, t47444, t47450)
}
