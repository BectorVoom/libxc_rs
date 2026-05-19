//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 429/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk429<F: Float>(t1107: F, t4322: F, t1101: F, t1109: F, t376: F, t1072: F, t1080: F, t1054: F, t1055: F, t1073: F, t1078: F, t1081: F, t1120: F, t1127: F, t396: F, t401: F, t403: F, t411: F, t4252: F, t4255: F, t4259: F, t4260: F, t4267: F, t4273: F, t4276: F, t4290: F, t4293: F, t4294: F, t4306: F, t4309: F, t4312: F, t4313: F, t4316: F, t4319: F) -> (F, F, F) {
    let t4324 = F::new(6.0) * t1107 * t4322;
    let t4325 = t1101 * t1109;
    let t4328 = F::cast_from(0.48245938496077605201e2_f64) * t1107 * t4325 * t376;
    let t4329 = t1072 * t1080;
    let t4333 = -t4252 + t4255 + t4259 + F::cast_from(0.68493333333333333332e-1_f64) * t1054 * t4260 * t403 - F::cast_from(0.51369999999999999999e-1_f64) * t1054 * t1055 * t1073 - F::cast_from(0.16522625736956710527e1_f64) * t1054 * t4267 * t1081 + F::cast_from(0.2069040516770936012e4_f64) * t4273 * t4276 - t4290 - F::cast_from(0.19298375398431042081e3_f64) * t4293 * t4294 + F::new(1.0) * t396 * t4306 + F::cast_from(0.35089341735807877242e1_f64) * t1127 * t4309 - F::cast_from(0.10389515463408878255e3_f64) * t4312 * t4313 + F::cast_from(0.5848223622634646207e0_f64) * t411 * t4316 - F::cast_from(0.35089341735807877242e1_f64) * t1120 * t4319 - t4324 - t4328 + F::cast_from(0.96491876992155210402e2_f64) * t1078 * t4329 * t401;
    (t4324, t4328, t4333)
}
