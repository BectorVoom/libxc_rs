//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 912/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk912(t33822: f64, t539: f64, t1375: f64, t32127: f64, t32154: f64, t33241: f64, t33247: f64, t33251: f64, t33274: f64, t33298: f64, t33798: f64, t33804: f64, t33810: f64, t33815: f64, t568: f64, t7194: f64, t7925: f64) -> (f64, f64) {
    let t33823 = t539 * t33822;
    let t33825 = -0.3289868133696452873e-1_f64 * t33241 + 2.0_f64 * t1375 * t33798 + 0.3289868133696452873e-1_f64 * t33247 + 0.6579736267392905746e-1_f64 * t33251 - t32127 + t32154 + 4.0_f64 * t1375 * t33804 + 0.6579736267392905746e-1_f64 * t33274 - 0.3289868133696452873e-1_f64 * t33298 - 6.0_f64 * t1375 * t33810 + 4.0_f64 * t7194 * t7925 + t33815 * t568 + t33823 * t568;
    (t33823, t33825)
}
