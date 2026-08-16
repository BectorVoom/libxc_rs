//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 126/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk126(t53: f64, t60: f64, t521: f64, t57: f64, t62: f64, zeta_threshold: f64) -> (f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t524 = piecewise3(t54, 0.0_f64, 4.0_f64 / 3.0_f64 * t57 * t521);
    let t525 = -t521;
    let t528 = piecewise3(t61, 0.0_f64, 4.0_f64 / 3.0_f64 * t62 * t525);
    let t529 = t524 + t528;
    (t525, t529)
}
