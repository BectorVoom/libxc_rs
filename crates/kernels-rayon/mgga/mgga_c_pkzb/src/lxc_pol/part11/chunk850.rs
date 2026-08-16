//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 850/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk850(t12: f64, t3363: f64, t5528: f64, t1837: f64, t3366: f64, t652: f64, t8729: f64, t1430: f64, t2732: f64, t439: f64, zeta_threshold: f64) -> (f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t9150 = t5528 * t3363;
    let t9155 = t1837 * t3366;
    let t9158 = t652 * t8729;
    let t9161 = piecewise3(t84, 0.0_f64, -28.0_f64 / 27.0_f64 * t9150 * t439 + 16.0_f64 / 9.0_f64 * t2732 * t1430 + 4.0_f64 / 9.0_f64 * t9155 * t439 - t9158 / 3.0_f64);
    (t9150, t9161)
}
