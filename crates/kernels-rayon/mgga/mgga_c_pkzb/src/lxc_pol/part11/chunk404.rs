//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 404/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk404(t146: f64, t1697: f64, t155: f64, t95: f64, t575: f64, t579: f64) -> (f64, f64, f64) {
    let t1698 = t146 * t1697;
    let t1701 = 35.0_f64 / 432.0_f64 * t1698 * t95 * t155;
    let t1702 = t575 * t579;
    (t1698, t1701, t1702)
}
