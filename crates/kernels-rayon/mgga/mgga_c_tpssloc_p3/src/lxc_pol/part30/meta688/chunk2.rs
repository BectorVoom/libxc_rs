//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2187/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2187(t24987: f64, t7756: f64, t2314: f64, t28025: f64, t4034: f64, t1266: f64, t28017: f64, t652: f64, t1845: f64, t5187: f64, t22574: f64, t8643: f64) -> (f64, f64, f64, f64, f64) {
    let t97779 = 2.0_f64 * t24987 * t7756;
    let t97783 = 2.0_f64 * t2314 * t28025;
    let t97785 = 2.0_f64 * t4034 * t28025;
    let t97788 = 2.0_f64 * t652 * t1266 * t28017;
    let t97789 = t5187 * t1845;
    let t97792 = 6.0_f64 * t22574 * t8643 * t97789;
    (t97779, t97783, t97785, t97788, t97792)
}
