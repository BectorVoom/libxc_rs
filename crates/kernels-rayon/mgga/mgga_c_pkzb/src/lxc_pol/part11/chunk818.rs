//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 818/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk818(t12: f64, t1430: f64, t2540: f64, t439: f64, t87: f64, t8721: f64, t8726: f64, t8729: f64, t3371: f64, t5106: f64, t1651: f64, t3374: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t8733 = piecewise3(t84, 0.0_f64, -8.0_f64 / 27.0_f64 * t8721 * t439 + 16.0_f64 / 9.0_f64 * t2540 * t1430 + 4.0_f64 / 9.0_f64 * t8726 * t439 + 4.0_f64 / 3.0_f64 * t87 * t8729);
    let t8734 = t5106 * t3371;
    let t8739 = t1651 * t3374;
    let t8742 = -t8729;
    (t8733, t8734, t8739, t8742)
}
