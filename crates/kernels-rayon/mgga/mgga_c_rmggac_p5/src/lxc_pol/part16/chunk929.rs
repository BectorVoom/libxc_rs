//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 929/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk929(t1614: f64, t3351: f64, t511: f64, t618: f64, t7231: f64, t10095: f64, t16043: f64, t1528: f64, t515: f64, t570: f64, t1652: f64, t34828: f64, t9864: f64) -> (f64, f64, f64, f64, f64) {
    let t45451 = t3351 * t7231 * t511 * t618 * t1614;
    let t45453 = t16043 * t10095;
    let t45458 = t3351 * t7231 * t515 * t1528 * t570;
    let t45463 = t3351 * t7231 * t515 * t618 * t1652;
    let t45466 = t34828 * t9864;
    (t45451, t45453, t45458, t45463, t45466)
}
