//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2360/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360(t16153: f64, t24995: f64, t8945: f64, t22574: f64, t25988: f64, t31035: f64, t2018: f64, t40611: f64, t1845: f64, t3698: f64, t26161: f64, t15868: f64, t1983: f64, t6996: f64) -> (f64, f64, f64, f64) {
    let t91681 = 6.0_f64 * t24995 * t8945 * t16153;
    let t91684 = 6.0_f64 * t22574 * t31035 * t25988;
    let t91686 = t2018 * t40611;
    let t91687 = t1845 * t3698;
    let t91690 = 6.0_f64 * t26161 * t91686 * t91687;
    let t91694 = 2.0_f64 * t1983 * t6996 * t15868;
    (t91681, t91684, t91690, t91694)
}
