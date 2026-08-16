//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 850/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk850(t4395: f64, t913: f64, t893: f64, t1556: f64, t2844: f64, t912: f64, t2842: f64, t2766: f64, t2848: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4396 = t4395 * t913;
    let t4398 = 1.0_f64 * t893 * t4396;
    let t4399 = t1556 * t2844;
    let t4400 = t4399 * t912;
    let t4402 = 0.16081979498692535067e2_f64 * t2842 * t4400;
    let t4408 = t2848 + 0.57077777777777777777e-2_f64 * t2766 + 0.57077777777777777777e-2_f64 * t4335 - 0.11415555555555555555e-1_f64 * t4340 + 0.34246666666666666666e-1_f64 * t4345 - 0.17123333333333333333e-1_f64 * t4349;
    (t4396, t4398, t4399, t4400, t4402, t4408)
}
