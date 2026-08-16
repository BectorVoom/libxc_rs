//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2264/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264(t2018: f64, t40611: f64, t1845: f64, t3698: f64, t26161: f64, t15868: f64, t1983: f64, t6996: f64, t3734: f64, t24995: f64, t8643: f64, t23831: f64, t7458: f64) -> (f64, f64, f64, f64) {
    let t91686 = t2018 * t40611;
    let t91687 = t1845 * t3698;
    let t91690 = 6.0_f64 * t26161 * t91686 * t91687;
    let t91694 = 2.0_f64 * t1983 * t6996 * t15868;
    let t91695 = t1845 * t3734;
    let t91698 = 6.0_f64 * t24995 * t8643 * t91695;
    let t91704 = 2.0_f64 * t7458 * t23831;
    (t91690, t91694, t91698, t91704)
}
