//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2191/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2191(t22666: f64, t26189: f64, t6888: f64, t22892: f64, t7691: f64, t80645: f64, t22633: f64, t22635: f64, t26337: f64, t3911: f64, t26206: f64, t6883: f64) -> (f64, f64, f64, f64) {
    let t90530 = t6888 * t22666 * t26189;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = 0.16449340668482264365e-1_f64 * t90533;
    let t90539 = t22633 * t22635 * t26337 * t3911;
    let t90541 = t6883 * t26206;
    (t90530, t90534, t90539, t90541)
}
