//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1424/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1424(t1862: f64, t2240: f64, t2244: f64, t607: f64, t2250: f64, t72: f64, t79: f64, t605: f64, t9259: f64, t39054: f64, t6489: f64, t39063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83814 = t2240 * t2244 * t1862;
    let t83817 = t607 * t1862;
    let t83820 = t72 * t79 * t2250;
    let t83822 = t605 * t9259;
    let t83827 = t39054 * t6489;
    let t83830 = t39063 * t6489;
    (t83814, t83817, t83820, t83822, t83827, t83830)
}
