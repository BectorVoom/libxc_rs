//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 913/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk913(t1719: f64, t2593: f64, t179: f64, t1721: f64, t6875: f64, t2583: f64, t5221: f64, t2586: f64, t568: f64, t581: f64, t1024: f64, t1692: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6903 = t2593 * t1719;
    let t6904 = t179 * t6903;
    let t6908 = t179 * t6875 * t1721;
    let t6914 = 7.0_f64 / 24.0_f64 * t5221 * t2583;
    let t6916 = t581 * t2586 * t568;
    let t6920 = t581 * t1024 * t1692;
    (t6903, t6904, t6908, t6914, t6916, t6920)
}
