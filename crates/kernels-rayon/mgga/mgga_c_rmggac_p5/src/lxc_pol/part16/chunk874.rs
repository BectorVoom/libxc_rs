//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 874/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk874(t874: f64, t9486: f64, t2447: f64, t4616: f64, t42023: f64, t42026: f64, t42044: f64, t42086: f64, t42101: f64, t40803: f64, t40831: f64, t40907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43970 = t874 * t9486;
    let t43974 = t4616 * t2447;
    let t43978 = 0.162600798888400151e-2_f64 * t42023;
    let t43979 = 0.162600798888400151e-2_f64 * t42026;
    let t43987 = 0.11918087970123395032e-3_f64 * t42044;
    let t44004 = 0.39726959900411316772e-4_f64 * t42086;
    let t44008 = 0.11918087970123395032e-3_f64 * t42101;
    let t44029 = 0.3193131120497015617e0_f64 * t40803;
    let t44035 = 0.3193131120497015617e0_f64 * t40831;
    let t44070 = 0.21819729323396273384e0_f64 * t40907;
    (t43970, t43974, t43978, t43979, t43987, t44004, t44008, t44029, t44035, t44070)
}
