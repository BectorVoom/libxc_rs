//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 859/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk859(t3351: f64, t515: f64, t5260: f64, t9188: f64, t1594: f64, t1986: f64, t7720: f64, t1627: f64, t3352: f64, t495: f64, t511: f64, t7230: f64) -> (f64, f64, f64) {
    let t39197 = t3351 * t9188 * t515 * t5260;
    let t39199 = t1986 * t1594;
    let t39200 = t7720 * t39199;
    let t39205 = t7230 * t3352 * t511 * t1627 * t495;
    (t39197, t39200, t39205)
}
