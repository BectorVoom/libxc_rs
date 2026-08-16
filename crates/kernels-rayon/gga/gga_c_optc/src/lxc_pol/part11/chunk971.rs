//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 971/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk971(t17361: f64, t17425: f64, t17429: f64, t17431: f64, t17433: f64, t17435: f64, t17438: f64, t17655: f64, t17658: f64, t17746: f64, t17753: f64, t17755: f64, t17758: f64, t17761: f64, t17764: f64, t17777: f64, t2935: f64, t2974: f64, t3035: f64, t3059: f64, t402: f64) -> f64 {
    let t17780 = -t17425 - t17429 - t17431 - t17433 - t17435 + t17438 - 0.19751789702565206229e-1_f64 * t17361 + t17655 - t17658 + t17746 - 0.35089340384731224426e1_f64 * t3035 * t17755 + 0.51947267698127589897e2_f64 * t3059 * t17758 - 6.0_f64 * t2935 * t17761 + 0.96494049533612093922e2_f64 * t2974 * t17764 + t17753 - 0.3109e-1_f64 * t17777 * t402;
    t17780
}
