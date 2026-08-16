//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 744/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk744(t2379: f64, t6638: f64, t6637: f64, t23035: f64, t6612: f64, t835: f64, t812: f64, t831: f64, t2686: f64, t6614: f64, t2627: f64, t59: f64) -> (f64, f64, f64, f64, f64) {
    let t23036 = t6638 * t2379;
    let t23037 = t6637 * t23036;
    let t23038 = t23035 * t23037;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23042 = t23041 * t831;
    let t23043 = 7.0_f64 / 1152.0_f64 * t23042;
    let t23044 = t6614 * t2686;
    let t23046 = t2627 * t59;
    (t23038, t23042, t23043, t23044, t23046)
}
