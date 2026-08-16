//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2101/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2101(t2240: f64, t2251: f64, t2261: f64, t43: f64, t2267: f64, t614: f64, t38: f64, t9287: f64, t835: f64, t1862: f64, t2244: f64, t39054: f64, t6489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83778 = t2240 * t2251;
    let t83788 = t2261 * t43;
    let t83791 = t614 * t2267;
    let t83796 = t38 * t9287;
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t83814 = t2240 * t2244 * t1862;
    let t83827 = t39054 * t6489;
    (t83778, t83788, t83791, t83796, t83803, t83814, t83827)
}
