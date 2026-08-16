//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 337/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk337(t22: f64, t698: f64, t656: f64, t3091: f64, t3100: f64, t3103: f64, t3197: f64, t3199: f64, t3200: f64) -> (f64, f64, f64) {
    let t3224 = t698 * t22;
    let t3225 = t3224 * t656;
    let t3281 = t3197 - 0.34093327067806677162e-2_f64 * t3091 + t3199 + t3200 - 0.9072038638458063915e-4_f64 * t3100 + 0.24108102678124669849e-4_f64 * t3103;
    (t3224, t3225, t3281)
}
