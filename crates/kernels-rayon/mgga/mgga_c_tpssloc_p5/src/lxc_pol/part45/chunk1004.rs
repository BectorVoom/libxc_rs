//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1004/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1004(t22581: f64, t8607: f64, t112611: f64, t1983: f64, t2095: f64, t22578: f64, t8640: f64, t31297: f64, t6876: f64, t31670: f64, t31650: f64, t6883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115271 = 2.0_f64 * t8607 * t22581;
    let t115275 = t1983 * t2095 * t112611;
    let t115277 = t1983 * t8640 * t22578;
    let t115279 = 2.0_f64 * t6876 * t31297;
    let t115283 = 2.0_f64 * t6876 * t31670;
    let t115292 = t6883 * t31650;
    (t115271, t115275, t115277, t115279, t115283, t115292)
}
