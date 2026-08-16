//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 712/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk712(t271: f64, t71: f64, t4789: f64, t1985: f64, t793: f64, t325: f64, t4685: f64, t1003: f64, t1171: f64, t226: f64, t3807: f64, t120: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20925 = t271 * t71;
    let t20963 = t4789 * t71;
    let t22971 = t1985 * t793;
    let t24363 = t4685 * t325;
    let t24889 = t1003 * t1003;
    let t24890 = 1.0_f64 / t24889;
    let t24983 = t1171 * t1171;
    let t24985 = 1.0_f64 / t226 / t24983;
    let t25441 = t3807 * t325;
    let t25518 = t120 * t860;
    (t20925, t20963, t22971, t24363, t24890, t24985, t25441, t25518)
}
