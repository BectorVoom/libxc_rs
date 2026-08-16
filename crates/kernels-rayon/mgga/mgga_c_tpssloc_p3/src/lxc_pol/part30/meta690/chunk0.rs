//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2199/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2199(t19534: f64, t89: f64, t1874: f64, t28030: f64, t6525: f64, t28821: f64, t6880: f64, t28239: f64, t6876: f64, t1983: f64, t26503: f64, t5161: f64) -> (f64, f64, f64, f64, f64) {
    let t97933 = t89 * t19534;
    let t97935 = 2.0_f64 * t97933 * t1874;
    let t97937 = 2.0_f64 * t28030 * t6525;
    let t97941 = 3.0_f64 * t28821 * t6880;
    let t97942 = t6876 * t28239;
    let t97947 = 2.0_f64 * t1983 * t26503 * t5161;
    (t97935, t97937, t97941, t97942, t97947)
}
