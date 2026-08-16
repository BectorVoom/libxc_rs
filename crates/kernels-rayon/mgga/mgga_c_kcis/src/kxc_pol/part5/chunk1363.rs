//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1363/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1363(t22430: f64, t6010: f64, t1529: f64, t7310: f64, t1494: f64, t21971: f64, t572: f64, t571: f64, t22411: f64, t22413: f64, t22415: f64, t22417: f64, t22420: f64, t22423: f64, t22425: f64, t22428: f64) -> (f64, f64, f64, f64) {
    let t22431 = t6010 * t22430;
    let t22433 = t1529 * t7310;
    let t22435 = t1494 * t21971;
    let t22436 = t572 * t22435;
    let t22437 = t571 * t22436;
    let t22439 = -t22411 / 72.0_f64 + t22413 / 96.0_f64 - t22415 / 128.0_f64 - t22417 / 12.0_f64 + 11.0_f64 / 27.0_f64 * t22420 - 19.0_f64 / 108.0_f64 * t22423 + t22425 / 128.0_f64 + 19.0_f64 / 144.0_f64 * t22428 - t22431 / 64.0_f64 - t22433 / 72.0_f64 + t22437 / 24.0_f64;
    (t22431, t22433, t22437, t22439)
}
