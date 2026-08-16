//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1062/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1062(t3180: f64, t3463: f64, t3275: f64, t3188: f64, t10328: f64, t12816: f64, t15796: f64, t3300: f64, t3301: f64, t3466: f64, t3468: f64, t3470: f64, t3475: f64, t3478: f64, t4571: f64, t4576: f64, t4579: f64, t5558: f64, t8: f64) -> f64 {
    let t15799 = 3.0_f64 * t3180;
    let t15800 = 3.0_f64 * t3463;
    let t15803 = 3.0_f64 * t3275;
    let t15804 = 6.0_f64 * t3188;
    let tv3rho30 = -3.0_f64 / 16.0_f64 * t5558 - 3.0_f64 / 16.0_f64 * t4576 - 3.0_f64 / 8.0_f64 * t4579 - 3.0_f64 / 8.0_f64 * t3466 - 3.0_f64 / 16.0_f64 * t4571 - 3.0_f64 / 16.0_f64 * t3475 - 3.0_f64 / 8.0_f64 * t3478 + 3.0_f64 / 8.0_f64 * t3468 + 3.0_f64 / 8.0_f64 * t3470 - t10328 + t8 * (t12816 + t15796) + t15799 + t15800 + 3.0_f64 * t3300 + 6.0_f64 * t3301 - t15803 + t15804;
    tv3rho30
}
