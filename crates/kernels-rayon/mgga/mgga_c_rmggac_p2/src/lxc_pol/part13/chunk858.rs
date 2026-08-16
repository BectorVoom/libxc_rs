//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 858/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk858(t2144: f64, t3351: f64, t3352: f64, t5263: f64, t1596: f64, t1986: f64, t7720: f64, t495: f64, t515: f64, t7230: f64, t8377: f64, t511: f64, t5169: f64, t9188: f64) -> (f64, f64, f64, f64) {
    let t39181 = t3351 * t3352 * t2144 * t5263;
    let t39183 = t1986 * t1596;
    let t39184 = t7720 * t39183;
    let t39189 = t7230 * t3352 * t515 * t8377 * t495;
    let t39193 = t3351 * t9188 * t511 * t5169;
    (t39181, t39184, t39189, t39193)
}
