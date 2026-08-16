//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 722/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk722(t13053: f64, t13636: f64, t13639: f64, t13643: f64, t13646: f64, t13650: f64, t13653: f64, t13655: f64, t13656: f64, t13660: f64, t13893: f64, t13895: f64) -> f64 {
    let t14402 = -0.57514388930881124514e0_f64 * t13636 + 0.9585731488480187419e0_f64 * t13639 - t13643 - t13646 - t13650 + t13653 + 0.38342925953920749676e1_f64 * t13053 - t13655 - t13656 + t13660 - 0.59584149919750711116e-1_f64 * t13893 + 0.59584149919750711116e-1_f64 * t13895;
    t14402
}
