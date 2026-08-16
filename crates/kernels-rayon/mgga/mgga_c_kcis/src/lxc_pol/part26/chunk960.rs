//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 960/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk960(t3255: f64, t7230: f64, t5463: f64, t5526: f64, t3786: f64, t1471: f64, t1472: f64, t18431: f64, t544: f64, t6957: f64, t1319: f64, t16411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22149 = t3255 * t7230;
    let t22151 = t5463 * t5526;
    let t22152 = t3786 * t22151;
    let t22156 = t1471 * t1472 * t18431;
    let t22159 = t544 * t6957;
    let t22160 = t22159 * t1319;
    let t22161 = t16411 * t22160;
    (t22149, t22151, t22152, t22156, t22160, t22161)
}
