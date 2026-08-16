//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 734/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk734(t4306: f64, t583: f64, t578: f64, t4246: f64, t4250: f64, t4252: f64, t4258: f64, t4263: f64, t4267: f64, t4271: f64, t4275: f64, t4279: f64, t4282: f64, t4284: f64, t4289: f64, t4295: f64, t4299: f64, t4304: f64) -> (f64, f64, f64) {
    let t4307 = t583 * t4306;
    let t4308 = t578 * t4307;
    let t4310 = t4246 / 16.0_f64 - t4250 / 8.0_f64 + t4252 / 12.0_f64 + t4258 / 8.0_f64 - t4263 / 12.0_f64 - t4267 / 16.0_f64 - t4271 / 72.0_f64 + t4275 / 24.0_f64 - t4279 / 256.0_f64 + t4282 / 128.0_f64 - t4284 / 96.0_f64 - t4289 / 128.0_f64 + t4295 / 96.0_f64 + t4299 / 256.0_f64 - t4304 / 576.0_f64 - t4308 / 192.0_f64;
    (t4307, t4308, t4310)
}
