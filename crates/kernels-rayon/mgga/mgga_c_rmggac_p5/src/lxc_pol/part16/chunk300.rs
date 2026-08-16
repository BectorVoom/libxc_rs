//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 300/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk300(t53: f64, t60: f64, t1794: f64, t1797: f64, t57: f64, t912: f64, t525: f64, t62: f64, t921: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1801 = piecewise3(t54, 0.0_f64, 4.0_f64 / 9.0_f64 * t912 * t1794 + 4.0_f64 / 3.0_f64 * t57 * t1797);
    let t1802 = t525 * t525;
    let t1805 = -t1797;
    let t1809 = piecewise3(t61, 0.0_f64, 4.0_f64 / 9.0_f64 * t921 * t1802 + 4.0_f64 / 3.0_f64 * t62 * t1805);
    let t1810 = t1801 + t1809;
    (t1802, t1805, t1810)
}
