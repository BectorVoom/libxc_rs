//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 107/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk107(t53: f64, t60: f64, t431: f64, t433: f64, t195: f64, t231: f64, t57: f64, t280: f64, t62: f64, t284: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t435 = 0.5848223622634646207e0_f64 * t431 * t433;
    let t436 = t195 * t231;
    let t437 = 1.0_f64 / t57;
    let t440 = piecewise3(t54, 0.0_f64, 2.0_f64 / 3.0_f64 * t437 * t280);
    let t441 = 1.0_f64 / t62;
    let t444 = piecewise3(t61, 0.0_f64, 2.0_f64 / 3.0_f64 * t441 * t284);
    let t446 = t440 / 2.0_f64 + t444 / 2.0_f64;
    (t435, t436, t437, t441, t446)
}
