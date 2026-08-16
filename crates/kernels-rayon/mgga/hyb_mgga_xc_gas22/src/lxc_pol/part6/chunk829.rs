//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 829/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk829(t1783: f64, t1819: f64, t555: f64, t1788: f64, t1797: f64, t10: f64, t1897: f64, t1806: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t6201 = t555 * t1819 * t1783;
    let t6204 = t555 * t1819 * t1788;
    let t6207 = t555 * t1819 * t1797;
    let t6209 = t1897 * t10;
    let t6214 = t550 * t1806;
    (t6201, t6204, t6207, t6209, t6214)
}
