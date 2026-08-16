//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1049/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1049(t2007: f64, t47854: f64, t1971: f64, t2144: f64, t30311: f64, t3351: f64, t46005: f64, t875: f64, t7720: f64, t9731: f64, t674: f64, t7715: f64, t9734: f64) -> (f64, f64, f64, f64, f64) {
    let t47857 = t47854 * t2007;
    let t47861 = t3351 * t1971 * t2144 * t30311;
    let t47866 = t3351 * t1971 * t875 * t46005;
    let t47868 = t7720 * t9731;
    let t47871 = t9734 * t7715 * t674;
    (t47857, t47861, t47866, t47868, t47871)
}
