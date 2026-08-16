//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 758/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk758(t404: f64, t6382: f64, t2411: f64, t53: f64, t2410: f64, t334: f64) -> (f64, f64, f64) {
    let t6383 = t404 * t6382;
    let t6398 = t53 * t2411;
    let t6404 = 1.0_f64 / t2410 / t334;
    (t6383, t6398, t6404)
}
