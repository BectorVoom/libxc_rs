//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 719/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk719(t45: f64, t4674: f64, t485: f64, t190: f64, t4579: f64, t681: f64, t1342: f64, t3572: f64, t4573: f64, t2337: f64, t3558: f64, t3561: f64, t741: f64, t80: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t4675 = t485 * t4674;
    let t4678 = t190 * t4579;
    let t4680 = 4.0_f64 * t681 * t4678;
    let t4682 = 8.0_f64 * t3572 * t1342;
    let t4683 = t190 * t4573;
    let t4685 = 12.0_f64 * t2337 * t4683;
    let t4686 = 0.11696447245269292414e1_f64 * t3558;
    let t4687 = 0.36622894612013090108e-3_f64 * t3561;
    let t4693 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t80 * t4573 + 2.0_f64 / 3.0_f64 * t741 * t4579);
    (t4675, t4678, t4680, t4682, t4683, t4685, t4686, t4687, t4693)
}
