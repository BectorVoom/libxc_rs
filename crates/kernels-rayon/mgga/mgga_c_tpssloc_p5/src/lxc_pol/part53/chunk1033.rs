//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1033/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1033(t117173: f64, t122168: f64, t122178: f64, t1323: f64, t1375: f64, t16439: f64, t1842: f64, t1843: f64, t2092: f64, t27115: f64, t32120: f64, t32150: f64, t32151: f64, t32156: f64, t33804: f64, t33822: f64, t3882: f64, t3887: f64, t5215: f64, t5321: f64, t5326: f64, t5354: f64, t568: f64, t7194: f64, t8794: f64, t93313: f64, t93338: f64) -> f64 {
    let t124069 = -0.3289868133696452873e-1_f64 * t122168 - 2.0_f64 * t7194 * t27115 - t117173 * t1843 - t5321 * t32151 + 2.0_f64 * t16439 * t8794 - t32120 * t5354 + 2.0_f64 * t1375 * t3887 * t32150 * t1842 + 4.0_f64 * t3882 * t33804 - 0.3289868133696452873e-1_f64 * t122178 - 2.0_f64 * t93338 * t2092 + t1323 * t33822 * t568 + 4.0_f64 * t5321 * t32156 - 2.0_f64 * t93313 * t2092 + 4.0_f64 * t5215 * t32156 + 2.0_f64 * t32120 * t5326;
    t124069
}
