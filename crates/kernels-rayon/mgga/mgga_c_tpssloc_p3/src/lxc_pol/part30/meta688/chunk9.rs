//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2194/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2194(t1388: f64, t6324: f64, t26161: f64, t91686: f64, t26504: f64, t7685: f64, t1983: f64, t22591: f64, t28834: f64, t19596: f64, t6996: f64, t24994: f64, t7684: f64) -> (f64, f64, f64, f64, f64) {
    let t97875 = t6324 * t1388;
    let t97878 = 6.0_f64 * t26161 * t91686 * t97875;
    let t97880 = 2.0_f64 * t7685 * t26504;
    let t97887 = 3.0_f64 * t1983 * t22591 * t28834;
    let t97889 = t1983 * t6996 * t19596;
    let t97890 = t7684 * t24994;
    (t97878, t97880, t97887, t97889, t97890)
}
