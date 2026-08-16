//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2623/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2623(t11734: f64, t15548: f64, t1174: f64, t14749: f64, t3431: f64, t1222: f64, t15723: f64, t11738: f64, t13969: f64, t15534: f64, t3514: f64, t53371: f64) -> (f64, f64, f64, f64, f64) {
    let t53378 = t11734 * t15548;
    let t53387 = t1174 * t3431 * t14749;
    let t53389 = t15723 * t1222;
    let t53397 = t11738 * t13969 * t15534;
    let t53399 = t53371 * t3514;
    (t53378, t53387, t53389, t53397, t53399)
}
