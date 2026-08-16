//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 717/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk717(t1375: f64, t2092: f64, t568: f64, t7194: f64, t8613: f64, t8623: f64, t8789: f64, t8794: f64, t8801: f64) -> f64 {
    let t8803 = 0.3289868133696452873e-1_f64 * t8613 - 0.3289868133696452873e-1_f64 * t8623 + t8789 * t568 - 2.0_f64 * t7194 * t2092 + 2.0_f64 * t1375 * t8794 - t1375 * t8801;
    t8803
}
