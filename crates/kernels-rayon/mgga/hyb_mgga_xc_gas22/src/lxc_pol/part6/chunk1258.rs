//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1258/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1258(t1134: f64, t7692: f64, t1139: f64, t7817: f64, t1171: f64, t9761: f64, t1605: f64, tau0: f64) -> (f64, f64, f64, f64) {
    let t26564 = t1134 * t7692;
    let t26579 = t7817 * t1139;
    let t26654 = t9761 * t1171;
    let t26727 = t1605 * tau0;
    (t26564, t26579, t26654, t26727)
}
