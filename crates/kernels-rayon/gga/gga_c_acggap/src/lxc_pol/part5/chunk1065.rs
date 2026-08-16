//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1065/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1065(t1603: f64, t323: f64, t851: f64, t315: f64, t5299: f64, t3101: f64, t316: f64, t449: f64, t556: f64, t322: f64, t5331: f64, t5368: f64, t868: f64) -> (f64, f64, f64, f64, f64) {
    let t18858 = t851 * t1603 * t323;
    let t18861 = t315 * t5299 * t323;
    let t18866 = t316 * t449 * t556 * t3101;
    let t18872 = t316 * t449 * t5331 * t322;
    let t18875 = t868 * t5368;
    (t18858, t18861, t18866, t18872, t18875)
}
