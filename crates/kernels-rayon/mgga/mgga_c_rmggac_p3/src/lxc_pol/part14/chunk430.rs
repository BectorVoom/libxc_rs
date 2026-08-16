//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 430/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk430(t1107: f64, t4322: f64, t1101: f64, t1109: f64, t376: f64, t1072: f64, t1080: f64, t1054: f64, t1055: f64, t1073: f64, t1078: f64, t1081: f64, t1120: f64, t1127: f64, t396: f64, t401: f64, t403: f64, t411: f64, t4252: f64, t4255: f64, t4259: f64, t4260: f64, t4267: f64, t4273: f64, t4276: f64, t4290: f64, t4293: f64, t4294: f64, t4306: f64, t4309: f64, t4312: f64, t4313: f64, t4316: f64, t4319: f64) -> (f64, f64, f64) {
    let t4324 = 6.0_f64 * t1107 * t4322;
    let t4325 = t1101 * t1109;
    let t4328 = 0.48245938496077605201e2_f64 * t1107 * t4325 * t376;
    let t4329 = t1072 * t1080;
    let t4333 = -t4252 + t4255 + t4259 + 0.68493333333333333332e-1_f64 * t1054 * t4260 * t403 - 0.51369999999999999999e-1_f64 * t1054 * t1055 * t1073 - 0.16522625736956710527e1_f64 * t1054 * t4267 * t1081 + 0.2069040516770936012e4_f64 * t4273 * t4276 - t4290 - 0.19298375398431042081e3_f64 * t4293 * t4294 + 1.0_f64 * t396 * t4306 + 0.35089341735807877242e1_f64 * t1127 * t4309 - 0.10389515463408878255e3_f64 * t4312 * t4313 + 0.5848223622634646207e0_f64 * t411 * t4316 - 0.35089341735807877242e1_f64 * t1120 * t4319 - t4324 - t4328 + 0.96491876992155210402e2_f64 * t1078 * t4329 * t401;
    (t4324, t4328, t4333)
}
