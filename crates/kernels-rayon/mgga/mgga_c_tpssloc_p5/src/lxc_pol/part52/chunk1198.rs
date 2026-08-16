//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1198/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1198(t1814: f64, t8465: f64, t8467: f64, t5248: f64, t5249: f64, t550: f64, t31170: f64, t1831: f64, t8466: f64, t31154: f64, t31161: f64, t31178: f64, t32712: f64, t32715: f64) -> (f64, f64, f64) {
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    let t32721 = t5248 * t5249 * t550;
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32726 = -t31154 - 0.48447307312968469025e-2_f64 * t32712 - t31161 - 0.80745512188280781708e-3_f64 * t32715 + t32718 / 1536.0_f64 - t32722 / 1536.0_f64 - t31178 - t32724 / 384.0_f64;
    (t32717, t32721, t32726)
}
