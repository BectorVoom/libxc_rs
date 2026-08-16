//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 928/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk928(t112797: f64, t30716: f64, t241: f64, t812: f64, t814: f64, t835: f64, t232: f64, t30714: f64, t4180: f64, t9626: f64, t9621: f64, t23046: f64, t2633: f64, t6605: f64) -> (f64, f64, f64, f64, f64) {
    let t112798 = t112797 * t30716;
    let t112802 = t812 * t814 * t835 * t241;
    let t112803 = t112802 * t30716;
    let t112807 = t30714 * t4180 * t9626 * t232;
    let t112811 = t30714 * t4180 * t9621 * t232;
    let t112814 = t6605 * t23046 * t2633;
    (t112798, t112803, t112807, t112811, t112814)
}
