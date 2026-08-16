//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 541/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk541(t14170: f64, t3851: f64, t14174: f64, t793: f64, t14207: f64, t305: f64, t1008: f64, t212: f64, t28: f64, t672: f64) -> (f64, f64, f64, f64, f64) {
    let t14349 = t3851 * t14170;
    let t14351 = t793 * t14174;
    let t14354 = 0.79828278012425390427e-1_f64 * t305 * t14207;
    let t14362 = t212 * t1008 * t28;
    let t14363 = t672 * t14362;
    (t14349, t14351, t14354, t14362, t14363)
}
