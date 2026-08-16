//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 890/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk890(t1378: f64, t32150: f64, t31595: f64, t2091: f64, t3887: f64, t7213: f64, t1375: f64, t1386: f64, t2092: f64, t24082: f64, t24095: f64, t31552: f64, t31561: f64, t31591: f64, t32120: f64, t32127: f64, t3758: f64, t3882: f64, t7194: f64, t7214: f64, t8801: f64) -> (f64, f64, f64, f64) {
    let t32151 = t1378 * t32150;
    let t32154 = 0.16449340668482264365e-1_f64 * t31595;
    let t32156 = t3887 * t2091 * t7213;
    let t32159 = 0.6579736267392905746e-1_f64 * t31552 - 2.0_f64 * t7194 * t7214 - t32120 * t1386 - 2.0_f64 * t24095 * t2092 - 2.0_f64 * t24082 * t2092 + 0.6579736267392905746e-1_f64 * t31561 - t32127 - t3758 * t8801 - t3882 * t8801 - t1375 * t32151 + 0.3289868133696452873e-1_f64 * t31591 + t32154 + 4.0_f64 * t1375 * t32156;
    (t32151, t32154, t32156, t32159)
}
