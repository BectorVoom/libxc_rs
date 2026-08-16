//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1413/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1413(t22685: f64, t26193: f64, t28191: f64, t1985: f64, t28232: f64, t107250: f64, t107260: f64, t20060: f64, t2016: f64, t20662: f64, t26366: f64, t28187: f64, t28220: f64, t5215: f64, t5321: f64, t6461: f64, t6958: f64, t74849: f64, t7750: f64, t81318: f64) -> f64 {
    let t107265 = t22685 * t26193 * t28191;
    let t107268 = t1985 * t26193 * t28232;
    let t107270 = -t81318 - t74849 * t2016 + 12.0_f64 * t5321 * t28220 - 0.49348022005446793095e-1_f64 * t107250 - 3.0_f64 * t26366 * t6461 - 3.0_f64 * t20060 * t7750 - t6958 * t20662 + 0.82246703342411321825e-2_f64 * t107260 - 3.0_f64 * t5215 * t28187 + 0.14804406601634037928e0_f64 * t107265 + 0.49348022005446793095e-1_f64 * t107268;
    t107270
}
