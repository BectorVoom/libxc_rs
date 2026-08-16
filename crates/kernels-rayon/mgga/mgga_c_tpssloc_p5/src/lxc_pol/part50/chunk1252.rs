//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1252/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1252(t1992: f64, t31091: f64, t90566: f64, t32698: f64, t6883: f64, t113946: f64, t1842: f64, t22635: f64, t32705: f64, t81159: f64, t113963: f64, t12021: f64, t1375: f64, t1385: f64, t16030: f64, t16460: f64, t31096: f64, t31131: f64, t32686: f64, t32690: f64, t32757: f64, t3758: f64, t3882: f64, t3887: f64, t5215: f64, t5321: f64, t5353: f64, t8475: f64, t8476: f64, t8485: f64, t8486: f64) -> f64 {
    let t120258 = 0.3289868133696452873e-1_f64 * t1992 * t90566 * t31091;
    let t120269 = t6883 * t32698;
    let t120270 = 0.38381794893125283518e-1_f64 * t120269;
    let t120274 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t113946 * t1842;
    let t120276 = t81159 * t32705;
    let t120277 = 0.76763589786250567037e-1_f64 * t120276;
    let t120292 = -6.0_f64 * t12021 * t1375 * t5353 * t8475 + 2.0_f64 * t1375 * t1385 * t32757 * t3887 + 2.0_f64 * t1375 * t3887 * t5353 * t8485 + 2.0_f64 * t16030 * t8476 - t16460 * t8486 + 4.0_f64 * t31096 * t5215 + 4.0_f64 * t31096 * t5321 + 2.0_f64 * t31131 * t5321 + 2.0_f64 * t32686 * t3758 - 6.0_f64 * t32690 * t3882 - t113963 + t120258 + t120270 + t120274 - t120277;
    t120292
}
