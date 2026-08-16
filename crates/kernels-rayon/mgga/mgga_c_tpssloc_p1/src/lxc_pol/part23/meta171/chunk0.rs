//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 784/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk784(t1891: f64, t67: f64, t246: f64, t2628: f64, t835: f64, t812: f64, t2690: f64, t815: f64, t116: f64, t126: f64, t136: f64, t16: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9688 = 1.0_f64 / t126 / t136 * t116 / 4.0_f64;
    let t9689 = t9688 * t16;
    (t9645, t9646, t9666, t9667, t9670, t9671, t9688, t9689)
}
