//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1366/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1366(t4072: f64, t652: f64, t8595: f64, t1983: f64, t27144: f64, t8643: f64, t31526: f64, t7685: f64, t33483: f64, t868: f64, t1914: f64, t26756: f64, t584: f64, t86730: f64) -> (f64, f64, f64, f64, f64) {
    let t121240 = 2.0_f64 * t652 * t8595 * t4072;
    let t121253 = t1983 * t27144 * t8643;
    let t121254 = t7685 * t31526;
    let t121258 = t33483 * t868;
    let t121264 = t26756 * t86730 * t584 * t1914;
    (t121240, t121253, t121254, t121258, t121264)
}
