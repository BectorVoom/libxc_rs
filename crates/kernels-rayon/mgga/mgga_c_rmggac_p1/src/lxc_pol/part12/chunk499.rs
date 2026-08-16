//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 499/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk499(t589: f64, t998: f64, t1143: f64, t1425: f64, t1535: f64, t4232: f64, t4252: f64, t4255: f64, t4259: f64, t4290: f64, t4351: f64, t4580: f64, t5412: f64, t5417: f64, t5418: f64, t5420: f64, t5421: f64, t5422: f64, t5426: f64, t5427: f64, t5429: f64, t5433: f64, t5435: f64, t5436: f64) -> (f64, f64) {
    let t5439 = t589 * t998;
    let t5442 = 0.186546e0_f64 * t5412 * t589 + t5417 + t5418 + t5420 - t5421 - 0.186546e0_f64 * t1425 * t5422 + t4232 + t4252 - t4255 - t4259 + t5426 - t4351 + t5427 + t5429 + t4290 + 0.93273e-1_f64 * t4580 * t1535 - t5433 + t5435 + 0.373092e0_f64 * t1143 * t5436 + 0.186546e0_f64 * t1143 * t5439;
    (t5439, t5442)
}
