//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 833/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk833(t164: f64, t51: f64, t592: f64, t8888: f64, t1727: f64, t3448: f64, t1020: f64, t179: f64, t6970: f64, t2575: f64, t2600: f64, t3441: f64) -> (f64, f64, f64, f64, f64) {
    let t8891 = t592 * t51 * t8888 * t164;
    let t8894 = t1727 * t3448;
    let t8897 = t179 * t6970 * t1020;
    let t8901 = t179 * t2600 * t2575;
    let t8904 = t3441 * t164;
    (t8891, t8894, t8897, t8901, t8904)
}
