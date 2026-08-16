//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1263/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1263(t1370: f64, t27596: f64, t7977: f64, t99247: f64, t27556: f64, t28778: f64, t3978: f64, t7984: f64, t98522: f64, t18171: f64, t28754: f64, t27583: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99320 = t1370 * t27596;
    let t99331 = t7977 * t99247;
    let t99392 = 0.30918233506944444444e-4_f64 * t27556 * t28778;
    let t99403 = t3978 * t7984;
    let t99411 = 0.15476481481481481481e-2_f64 * t98522;
    let t99422 = t18171 * t28754;
    let t99424 = 0.7722800925925925926e-4_f64 * t27583 * t99422;
    (t99320, t99331, t99392, t99403, t99411, t99422, t99424)
}
