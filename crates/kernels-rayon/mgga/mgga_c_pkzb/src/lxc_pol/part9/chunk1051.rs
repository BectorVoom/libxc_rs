//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1051/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1051(t16369: f64, t5227: f64, t149: f64, t1773: f64, t95: f64, t5402: f64, t579: f64, t583: f64, t1712: f64, t5264: f64, t1698: f64, t1705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16370 = t16369 * t5227;
    let t16373 = t149 * t95 * t1773;
    let t16378 = t5402 * t579;
    let t16379 = t16378 * t583;
    let t16381 = t5264 * t1712;
    let t16388 = t1698 * t1705;
    (t16370, t16373, t16378, t16379, t16381, t16388)
}
