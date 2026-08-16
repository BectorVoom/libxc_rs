//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2314/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314(t52: f64, t12961: f64, t1431: f64, t16558: f64, t16649: f64, t17635: f64, t20217: f64, t20234: f64, t2298: f64, t3966: f64, t4111: f64, t5398: f64, t607: f64, t67060: f64, t771: f64, t78: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t67280 = piecewise3(t150, 0.0_f64, -56.0_f64 / 81.0_f64 * t2298 * t20234 * t607 - 8.0_f64 / 9.0_f64 * t16649 * t3966 - 8.0_f64 / 9.0_f64 * t1431 * t17635 - 2.0_f64 / 3.0_f64 * t12961 * t5398 - 2.0_f64 / 3.0_f64 * t4111 * t16558 - 2.0_f64 / 9.0_f64 * t78 * t20217 * t607 - 2.0_f64 / 3.0_f64 * t771 * t67060);
    t67280
}
