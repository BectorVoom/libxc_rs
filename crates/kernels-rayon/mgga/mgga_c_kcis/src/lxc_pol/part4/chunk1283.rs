//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1283/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1283(t3820: f64, t509: f64, t3781: f64, t5458: f64, t1409: f64, t1897: f64, t3815: f64, t3786: f64, t518: f64, t5526: f64, t1319: f64, t3809: f64, t5493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16411 = t509 * t3820;
    let t16412 = t5458 * t3781;
    let t16413 = t16411 * t16412;
    let t16416 = t1409 * t1897;
    let t16417 = t16416 * t3815;
    let t16418 = t3786 * t16417;
    let t16421 = t518 * t5526;
    let t16422 = t16421 * t1319;
    let t16423 = t3786 * t16422;
    let t16426 = t5493 * t3809;
    (t16412, t16413, t16417, t16418, t16422, t16423, t16426)
}
