//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1464/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464(t41306: f64, t41292: f64, t41299: f64, t41303: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41361: f64, t41363: f64, t41369: f64, t41373: f64, t41384: f64, t41387: f64) -> f64 {
    let t41690 = 0.5356037037037037037e1_f64 * t41306;
    let t41701 = 0.12349037037037037037e0_f64 * t41292 - 0.10805407407407407407e0_f64 * t41299 - 0.104195e0_f64 * t41303 + t41690 + 0.6311625e0_f64 * t41373 - 0.15302962962962962963e1_f64 * t41341 - 0.516475e0_f64 * t41344 - 0.123954e2_f64 * t41347 + 0.68863333333333333334e1_f64 * t41350 + 0.21424148148148148148e1_f64 * t41361 + 0.27545333333333333333e1_f64 * t41363 - 0.27545333333333333332e1_f64 * t41369 + 0.2366859375e0_f64 * t41384 + 0.94674375e0_f64 * t41387;
    t41701
}
