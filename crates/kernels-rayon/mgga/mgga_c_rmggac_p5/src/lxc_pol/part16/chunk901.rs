//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 901/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk901(t5055: f64, t8407: f64, t2283: f64, t38638: f64, t1737: f64, t2084: f64, t27: f64, t7273: f64, t3351: f64, t511: f64, t6382: f64, t9188: f64) -> (f64, f64, f64, f64) {
    let t44956 = t5055 * t8407;
    let t44977 = t38638 * t2283;
    let t44982 = t7273 * t27 * t2084 * t1737;
    let t44986 = t3351 * t9188 * t511 * t6382;
    (t44956, t44977, t44982, t44986)
}
