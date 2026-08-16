//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2017/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2017(t608: f64, t9239: f64, t22522: f64, t2267: f64, t614: f64, t38: f64, t9287: f64, t835: f64, t39054: f64, t6489: f64, t39063: f64, t531: f64, t6995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83717 = t9239 * t608;
    let t83741 = t9239 * t22522;
    let t83791 = t614 * t2267;
    let t83796 = t38 * t9287;
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t83827 = t39054 * t6489;
    let t83830 = t39063 * t6489;
    let t83859 = t531 * t6995;
    (t83717, t83741, t83791, t83796, t83803, t83827, t83830, t83859)
}
