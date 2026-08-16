//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 339/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk339(t118: f64, t3282: f64, t3074: f64, t3078: f64, t3184: f64, t3185: f64, t3187: f64, t3190: f64, t3193: f64, t3196: f64) -> f64 {
    let t3283 = t118 * t3282;
    let t3285 = -t3184 + t3185 - t3190 - 0.31062809106223861414e-2_f64 * t3078 + t3193 - t3074 + t3187 - t3196 + 0.19957069503106347607e-1_f64 * t3283;
    t3285
}
