//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1019/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1019(t21347: f64, t324: f64, t10832: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t21124: f64, t21128: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64) {
    let t21348 = t21347 * t324;
    let t21360 = -t10832 - 0.2283111111111111111e-1_f64 * t13598 + 0.11415555555555555555e-1_f64 * t17149 - 0.34246666666666666665e-1_f64 * t17165 + 0.17123333333333333333e-1_f64 * t17175 - 0.19025925925925925925e-1_f64 * t21147 + 0.68493333333333333331e-1_f64 * t21150 - 0.34246666666666666665e-1_f64 * t21124 - 0.10274e0_f64 * t21153 + 0.10274e0_f64 * t21128 - 0.17123333333333333333e-1_f64 * t21156;
    (t21348, t21360)
}
