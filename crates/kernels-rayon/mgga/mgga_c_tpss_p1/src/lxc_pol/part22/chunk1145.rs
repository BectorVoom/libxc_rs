//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1145/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1145(t30: f64, t12767: f64, t737: f64, t9969: f64, t12727: f64, t187: f64, t10016: f64, t10022: f64, t1288: f64, t9924: f64, t2: f64, t3217: f64, t1197: f64, t12700: f64, t1991: f64, t22: f64, t3218: f64, t4380: f64, t4383: f64, t555: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t12769 = 0.11696447245269292414e1_f64 * t12767 * t737;
    let t12770 = 0.18311447306006545054e-3_f64 * t9969;
    let t12775 = 0.19751673498613801407e-1_f64 * t12727 * t187;
    let t12779 = 24.0_f64 * t10016;
    let t12780 = 48.0_f64 * t10022;
    let t12781 = t9924 * t1288;
    let t12784 = t3217 * t2;
    let t12794 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t12781 * t3218 - 8.0_f64 / 9.0_f64 * t12784 * t12700 - 2.0_f64 / 9.0_f64 * t4380 * t1991 + 4.0_f64 / 3.0_f64 * t1197 * t555 - 4.0_f64 * t4383 * t22);
    (t12769, t12770, t12775, t12779, t12780, t12794)
}
