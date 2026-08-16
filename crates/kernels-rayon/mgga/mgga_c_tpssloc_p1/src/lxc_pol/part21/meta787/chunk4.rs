//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2742/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742(t52: f64, t12606: f64, t12652: f64, t12874: f64, t16558: f64, t16563: f64, t16568: f64, t2244: f64, t2250: f64, t2440: f64, t40647: f64, t4087: f64, t5392: f64, t5398: f64, t55677: f64, t55723: f64, t607: f64, t76: f64, t9438: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t57873 = piecewise3(t150, 0.0_f64, 40.0_f64 / 81.0_f64 * t40647 * t5392 * t2244 + 32.0_f64 / 27.0_f64 * t12874 * t12652 + 8.0_f64 / 27.0_f64 * t16563 * t2250 + 8.0_f64 / 9.0_f64 * t2440 * t55723 + 8.0_f64 / 9.0_f64 * t4087 * t12606 + 8.0_f64 / 27.0_f64 * t9438 * t5398 * t2244 + 8.0_f64 / 9.0_f64 * t2440 * t16558 * t607 + 4.0_f64 / 9.0_f64 * t16568 * t2250 - 4.0_f64 / 3.0_f64 * t76 * t55677);
    t57873
}
