//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1007/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1007(t1441: f64, t7467: f64, t2040: f64, t33211: f64, t7796: f64, t102386: f64, t1874: f64, t28239: f64, t8607: f64, t22574: f64, t28830: f64, t36740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128296 = t1441 * t7467;
    let t128298 = 4.0_f64 * t128296 * t2040;
    let t128300 = 4.0_f64 * t33211 * t7796;
    let t128302 = 2.0_f64 * t102386 * t1874;
    let t128303 = t8607 * t28239;
    let t128306 = 6.0_f64 * t22574 * t36740 * t28830;
    (t128296, t128298, t128300, t128302, t128303, t128306)
}
