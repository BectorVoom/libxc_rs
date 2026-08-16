//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1034/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1034(t10628: f64, t549: f64, t6111: f64, t24505: f64, t2684: f64, t9438: f64, t3271: f64, t8634: f64, t10050: f64, t3040: f64, t1457: f64, t2103: f64, t43213: f64) -> (f64, f64, f64, f64, f64) {
    let t43715 = t6111 * t549 * t10628;
    let t43716 = 0.11916829983950142223e0_f64 * t43715;
    let t43718 = t2684 * t9438 * t24505;
    let t43719 = 0.7988109573733489516e-1_f64 * t43718;
    let t43721 = 0.35750489951850426669e0_f64 * t3271 * t8634;
    let t43723 = 0.35750489951850426669e0_f64 * t10050 * t3040;
    let t43726 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t43213;
    (t43716, t43719, t43721, t43723, t43726)
}
