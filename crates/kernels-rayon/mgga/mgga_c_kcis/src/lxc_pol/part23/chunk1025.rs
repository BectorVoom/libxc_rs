//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1025/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1025(t26602: f64, t7592: f64, t7583: f64, t7579: f64, t9229: f64, t209: f64, t2415: f64, t7581: f64) -> (f64, f64, f64, f64, f64) {
    let t26603 = t26602 * t7592;
    let t26605 = t26602 * t7583;
    let t26607 = t9229 * t7579;
    let t26608 = t26607 * t7583;
    let t26611 = t209 * t7581 * t2415;
    (t26603, t26605, t26607, t26608, t26611)
}
