//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1053/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1053(t5794: f64, t950: f64, t5791: f64, t10556: f64, t10832: f64, t13563: f64, t13598: f64, t14409: f64, t14410: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64) -> (f64, f64, f64) {
    let t17451 = t5794 * t950;
    let t17454 = t5791 * t950;
    let t17471 = -t10832 - 0.76103703703703703703e-2_f64 * t10556 - 0.1522074074074074074e-1_f64 * t13598 + 0.761037037037037037e-2_f64 * t13563 - t14409 + t14410 + 0.3805185185185185185e-2_f64 * t17149 - 0.19025925925925925925e-1_f64 * t17154 + 0.68493333333333333331e-1_f64 * t17159 - 0.2283111111111111111e-1_f64 * t17163 - 0.11415555555555555555e-1_f64 * t17165 - 0.10274e0_f64 * t17169 + 0.68493333333333333332e-1_f64 * t17173 + 0.57077777777777777777e-2_f64 * t17175 - 0.11415555555555555555e-1_f64 * t17180 + 0.34246666666666666666e-1_f64 * t17185 - 0.17123333333333333333e-1_f64 * t17189;
    (t17451, t17454, t17471)
}
