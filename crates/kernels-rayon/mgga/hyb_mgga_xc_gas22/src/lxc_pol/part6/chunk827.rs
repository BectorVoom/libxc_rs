//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 827/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk827(t6091: f64, t1967: f64, t81: f64, t1952: f64, t622: f64, t72: f64, t1815: f64, t557: f64) -> (f64, f64, f64, f64, f64) {
    let t6092 = 1.0_f64 / t6091;
    let t6096 = t81 * t1967;
    let t6116 = 1.0_f64 / t1952 / t622;
    let t6127 = 1.0_f64 / t6091 / t72;
    let t6160 = t1815 * t557;
    (t6092, t6096, t6116, t6127, t6160)
}
