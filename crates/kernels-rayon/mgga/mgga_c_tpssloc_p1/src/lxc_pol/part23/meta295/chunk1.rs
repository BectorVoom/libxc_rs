//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1015/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1015(t21334: f64, t291: f64, t10608: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t21124: f64, t21128: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64) {
    let t21336 = 0.621814e-1_f64 * t21334 * t291;
    let t21347 = -t10608 - 0.12361111111111111111e-1_f64 * t13598 + 0.61805555555555555556e-2_f64 * t17149 - 0.18541666666666666667e-1_f64 * t17165 + 0.92708333333333333334e-2_f64 * t17175 - 0.10300925925925925926e-1_f64 * t21147 + 0.37083333333333333333e-1_f64 * t21150 - 0.18541666666666666666e-1_f64 * t21124 - 0.55625000000000000001e-1_f64 * t21153 + 0.55625000000000000001e-1_f64 * t21128 - 0.92708333333333333333e-2_f64 * t21156;
    (t21336, t21347)
}
