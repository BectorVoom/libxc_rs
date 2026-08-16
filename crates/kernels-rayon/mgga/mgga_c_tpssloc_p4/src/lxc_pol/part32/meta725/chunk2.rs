//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2331/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2331(t28525: f64, t461: f64, t7324: f64, t18342: f64, t18346: f64, t18590: f64, t18594: f64, t18997: f64, t19068: f64, t27604: f64, t27617: f64, t27674: f64, t4974: f64, t4989: f64, t5046: f64, t7310: f64, t7331: f64, t7345: f64, t95550: f64, t95571: f64, t95573: f64) -> f64 {
    let t104387 = t7324 * t28525 * t461;
    let t104404 = t95550 / 5184.0_f64 + t27674 * t5046 / 54.0_f64 - t7310 * t18997 / 288.0_f64 - 0.10093189023535097714e-3_f64 * t104387 * t7331 + t95571 - t95573 + 5.0_f64 / 3456.0_f64 * t7345 * t18342 + 5.0_f64 / 1152.0_f64 * t7345 * t18346 - t7345 * t18590 / 576.0_f64 - t7345 * t18594 / 384.0_f64 + 5.0_f64 / 6912.0_f64 * t7345 * t19068 - 5.0_f64 / 648.0_f64 * t27604 * t4989 - t27617 * t4974 / 576.0_f64;
    t104404
}
