//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1774/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1774(t154: f64, t8705: f64, t1887: f64, t534: f64, t12267: f64, t6951: f64, t131: f64, t22791: f64, t9537: f64, t1338: f64, t225: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    let t80849 = t12267 * t6951;
    let t80853 = t22791 * t131 * t9537;
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    (t80845, t80847, t80849, t80853, t80854, t80855)
}
