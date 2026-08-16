//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2301/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2301(t40: f64, t12862: f64, t12865: f64, t16549: f64, t16558: f64, t17635: f64, t20217: f64, t20234: f64, t2433: f64, t3966: f64, t40632: f64, t4080: f64, t5398: f64, t607: f64, t67060: f64, t73: f64, zeta_threshold: f64) -> f64 {
    let t146 = t40 <= zeta_threshold;
    let t67064 = piecewise3(t146, 0.0_f64, 40.0_f64 / 81.0_f64 * t40632 * t20234 * t607 - 8.0_f64 / 9.0_f64 * t16549 * t3966 - 8.0_f64 / 9.0_f64 * t12862 * t17635 + 4.0_f64 / 3.0_f64 * t12865 * t5398 + 4.0_f64 / 3.0_f64 * t4080 * t16558 + 4.0_f64 / 9.0_f64 * t2433 * t20217 * t607 + 4.0_f64 / 3.0_f64 * t73 * t67060);
    t67064
}
