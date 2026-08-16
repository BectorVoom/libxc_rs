//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 421/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk421(t1662: f64, t83: f64, t124: f64, t1661: f64, t123: f64, t148: f64, t475: f64) -> (f64, f64, f64) {
    let t1663 = t83 * t1662;
    let t1665 = 0.19751673498613801407e-1_f64 * t1661 * t124;
    let t1667 = t475 * t148 * t123;
    (t1663, t1665, t1667)
}
