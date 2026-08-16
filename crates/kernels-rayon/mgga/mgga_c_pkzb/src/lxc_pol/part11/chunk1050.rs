//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1050/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1050(t174: f64, t46: f64, t2590: f64, t5278: f64, t5224: f64, t575: f64, t149: f64, t1773: f64, t95: f64, t5402: f64, t579: f64, t583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16322 = t174 * t174;
    let t16323 = 1.0_f64 / t16322;
    let t16324 = t16323 * t46;
    let t16343 = t2590 * t5278;
    let t16369 = t575 * t5224;
    let t16373 = t149 * t95 * t1773;
    let t16378 = t5402 * t579;
    let t16379 = t16378 * t583;
    (t16324, t16343, t16369, t16373, t16378, t16379)
}
