//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 824/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk824(t12: f64, t24: f64, t5158: f64, t1064: f64, t1430: f64, t207: f64, t3510: f64, t3512: f64, t439: f64, t8729: f64, t1165: f64, t333: f64, t3725: f64, t3727: f64, t507: f64, t8742: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t8795 = 0.17315859105681463759e2_f64 * t5158;
    let t8805 = piecewise3(t84, 0.0_f64, 8.0_f64 / 27.0_f64 * t3510 * t439 - 8.0_f64 / 9.0_f64 * t1064 * t1430 - 2.0_f64 / 9.0_f64 * t3512 * t439 + 2.0_f64 / 3.0_f64 * t207 * t8729);
    let t8815 = piecewise3(t90, 0.0_f64, 8.0_f64 / 27.0_f64 * t3725 * t507 + 8.0_f64 / 9.0_f64 * t1165 * t1430 - 2.0_f64 / 9.0_f64 * t3727 * t507 + 2.0_f64 / 3.0_f64 * t333 * t8742);
    (t8795, t8805, t8815)
}
