//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 715/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk715(t1698: f64, t579: f64, t583: f64, t1731: f64, t1773: f64, t1730: f64) -> (f64, f64, f64, f64) {
    let t5264 = t1698 * t579;
    let t5265 = t5264 * t583;
    let t5278 = t1731 * t1773;
    let t5279 = t1730 * t5278;
    (t5264, t5265, t5278, t5279)
}
