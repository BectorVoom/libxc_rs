//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 914/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk914(t1378: f64, t33843: f64, t1375: f64, t1843: f64, t2092: f64, t27009: f64, t27068: f64, t32120: f64, t32173: f64, t32183: f64, t33308: f64, t33311: f64, t5215: f64, t5321: f64, t7194: f64, t7937: f64, t8794: f64, t8801: f64) -> (f64, f64) {
    let t33844 = t1378 * t33843;
    let t33852 = -t32120 * t1843 - 2.0_f64 * t27009 * t2092 - t5215 * t8801 - t5321 * t8801 - 0.6579736267392905746e-1_f64 * t33308 - 0.3289868133696452873e-1_f64 * t33311 - 2.0_f64 * t7194 * t7937 + t32173 - t32183 - t1375 * t33844 + 2.0_f64 * t5215 * t8794 + 2.0_f64 * t5321 * t8794 - 2.0_f64 * t27068 * t2092;
    (t33844, t33852)
}
