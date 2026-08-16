//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1218/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1218(t1361: f64, t22690: f64, t3734: f64, t80840: f64, t154: f64, t8705: f64, t1887: f64, t534: f64, t12267: f64, t6951: f64, t1369: f64, t131: f64, t22791: f64, t9537: f64) -> (f64, f64, f64, f64, f64) {
    let t80843 = t80840 * t22690 * t1361 * t3734;
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    let t80848 = 455.0_f64 / 1296.0_f64 * t80847;
    let t80849 = t12267 * t6951;
    let t80850 = t80849 * t1369;
    let t80853 = t22791 * t131 * t9537;
    (t80843, t80845, t80848, t80850, t80853)
}
