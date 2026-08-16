//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1165/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1165(t1165: f64, t39753: f64, t604: f64, t7337: f64, t31421: f64, t35570: f64, t35574: f64, t35581: f64, t35586: f64, t35595: f64, t35597: f64, t35602: f64, t35609: f64, t35611: f64, t35617: f64, t37622: f64, t40063: f64, t40068: f64, t40072: f64, t40076: f64) -> f64 {
    let t40080 = t7337 * t1165 * t604 * t39753;
    let t40082 = t35570 + 0.21437009059034868486e-2_f64 * t40063 - t35574 + t35581 - t35586 + t37622 + t35595 + t35597 + t35602 + 0.114609375e-1_f64 * t31421 - 0.85748036236139473944e-3_f64 * t40068 - 0.10718504529517434243e-3_f64 * t40072 - 0.15724046144802076034e-3_f64 * t40076 - 0.7862023072401038017e-3_f64 * t40080 + t35609 + t35611 - t35617;
    t40082
}
