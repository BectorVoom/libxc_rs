//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 966/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk966(t16891: f64, t232: f64, t30714: f64, t4180: f64, t112792: f64, t16839: f64, t2632: f64, t5617: f64, t6605: f64, t6612: f64, t5593: f64, t5575: f64, t8342: f64, t8344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126298 = t30714 * t4180 * t16891 * t232;
    let t126302 = t112792 * t4180 * t16839 * t2632;
    let t126306 = t30714 * t4180 * t16839 * t232;
    let t126309 = t6605 * t6612 * t5617;
    let t126312 = t30714 * t5593;
    let t126316 = t5575 * t8342 * t8344;
    (t126298, t126302, t126306, t126309, t126312, t126316)
}
