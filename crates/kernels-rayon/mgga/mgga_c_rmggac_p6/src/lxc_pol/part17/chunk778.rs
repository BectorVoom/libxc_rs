//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 778/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk778(t36935: f64, t7936: f64, t14267: f64, t71: f64, t132: f64, t270: f64, t31: f64, t35688: f64, t1323: f64, t1326: f64, t35253: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t36936 = t36935 * t7936;
    let t36938 = t14267 * t71;
    let t36940 = t132 * t270 * t31;
    let t36942 = t35688 * t36938 * t36940;
    let t36943 = 0.13010691197123848594e-3_f64 * t36942;
    let t36945 = t1323 * t1326 * t14267;
    let t36948 = t36945 * t35253 * t68 * t36940;
    (t36936, t36940, t36943, t36945, t36948)
}
