//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2759/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759(t40: f64, t12606: f64, t12652: f64, t1430: f64, t16558: f64, t16637: f64, t16642: f64, t2244: f64, t2250: f64, t4104: f64, t5433: f64, t5435: f64, t55677: f64, t55723: f64, t607: f64, t75: f64, t767: f64, zeta_threshold: f64) -> f64 {
    let t146 = t40 <= zeta_threshold;
    let t58116 = piecewise3(t146, 0.0_f64, -56.0_f64 / 81.0_f64 * t5433 * t2244 + 32.0_f64 / 27.0_f64 * t1430 * t12652 + 8.0_f64 / 27.0_f64 * t16637 * t2250 - 4.0_f64 / 9.0_f64 * t75 * t55723 - 4.0_f64 / 9.0_f64 * t4104 * t12606 + 8.0_f64 / 27.0_f64 * t5435 * t2244 - 4.0_f64 / 9.0_f64 * t75 * t16558 * t607 - 2.0_f64 / 9.0_f64 * t16642 * t2250 + 2.0_f64 / 3.0_f64 * t767 * t55677);
    t58116
}
