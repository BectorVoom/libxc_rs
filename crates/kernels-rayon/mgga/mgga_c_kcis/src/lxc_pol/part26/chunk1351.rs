//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1351/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1351(t1385: f64, t28351: f64, t58540: f64, t101868: f64, t103063: f64, t103066: f64, t103069: f64, t103073: f64, t103078: f64, t16884: f64, t27369: f64, t3984: f64, t59380: f64, t59578: f64, t7908: f64, t7909: f64, t7911: f64, t97997: f64, t98016: f64) -> (f64, f64) {
    let t103083 = t28351 * t58540 * t1385;
    let t103095 = -0.2782641015625e-3_f64 * t27369 * t103063 + 0.41188271604938271607e-3_f64 * t103066 - 0.556528203125e-3_f64 * t27369 * t103069 - 0.46336805555555555557e-3_f64 * t7908 * t103073 - 0.22109259259259259259e-2_f64 * t97997 - 0.23168402777777777778e-3_f64 * t103078 * t7911 + 0.41188271604938271607e-3_f64 * t98016 + 0.41703125000000000001e-2_f64 * t7908 * t103083 - 0.33163888888888888888e-2_f64 * t101868 + 0.46336805555555555556e-3_f64 * t7908 * t3984 * t7909 * t59578 - 0.92673611111111111112e-3_f64 * t7908 * t16884 * t7909 * t59380;
    (t103083, t103095)
}
