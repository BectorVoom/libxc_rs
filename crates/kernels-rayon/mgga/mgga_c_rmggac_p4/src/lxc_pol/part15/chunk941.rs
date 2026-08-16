//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 941/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk941(t17859: f64, t9184: f64, t10194: f64, t290: f64, t3351: f64, t515: f64, t6561: f64, t9188: f64, t2144: f64, t3352: f64, t6564: f64, t6523: f64, t875: f64) -> (f64, f64, f64, f64, f64) {
    let t45648 = t17859 * t9184;
    let t45651 = t290 * t10194;
    let t45656 = t3351 * t9188 * t515 * t6561;
    let t45660 = t3351 * t3352 * t2144 * t6564;
    let t45664 = t3351 * t3352 * t875 * t6523;
    (t45648, t45651, t45656, t45660, t45664)
}
