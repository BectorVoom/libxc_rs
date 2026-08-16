//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 970/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk970(t7033: f64, t7038: f64, t7040: f64, t10536: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5025: f64, t5178: f64, t5186: f64) -> (f64, f64, f64, f64) {
    let t10592 = 0.51947577317044391276e2_f64 * t7033;
    let t10593 = 0.17544670867903938621e1_f64 * t7038;
    let t10594 = 0.35089341735807877242e1_f64 * t7040;
    let t10595 = t10536 + t4996 + t5005 - t5011 - t10592 - t10593 + t10594 + t5019 - t5022 + t5178 + t5186 + t5025;
    (t10592, t10593, t10594, t10595)
}
