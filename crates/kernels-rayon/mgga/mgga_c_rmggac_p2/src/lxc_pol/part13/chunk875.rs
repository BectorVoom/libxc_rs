//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 875/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk875(t5016: f64, t9000: f64, t16043: f64, t8812: f64, t2320: f64, t35146: f64, t7691: f64, t8616: f64, t27146: f64, t3351: f64, t3352: f64, t515: f64) -> (f64, f64, f64, f64, f64) {
    let t39451 = t5016 * t9000;
    let t39453 = t16043 * t8812;
    let t39455 = t35146 * t2320;
    let t39457 = t7691 * t8616;
    let t39461 = t3351 * t3352 * t515 * t27146;
    (t39451, t39453, t39455, t39457, t39461)
}
