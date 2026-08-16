//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 747/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk747(t1692: f64, t1734: f64, t179: f64, t1753: f64, t600: f64, t164: f64, t1732: f64, t2590: f64) -> (f64, f64, f64, f64, f64) {
    let t5236 = t179 * t1734 * t1692;
    let t5239 = t1753 * t600;
    let t5240 = t5239 * t164;
    let t5241 = t179 * t5240;
    let t5244 = t2590 * t1732;
    (t5236, t5239, t5240, t5241, t5244)
}
