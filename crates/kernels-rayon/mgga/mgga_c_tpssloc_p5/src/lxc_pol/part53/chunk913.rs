//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 913/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk913(t1825: f64, t32136: f64, t33822: f64, t553: f64, t1336: f64, t1814: f64, t32130: f64, t32132: f64, t33278: f64, t33282: f64, t33286: f64, t544: f64, t8798: f64) -> (f64, f64, f64) {
    let t33839 = t32136 * t1825;
    let t33841 = t553 * t33822;
    let t33843 = -t32130 - 0.6579736267392905746e-1_f64 * t33278 - t32132 - 0.3289868133696452873e-1_f64 * t33282 + 0.3289868133696452873e-1_f64 * t33286 + t1814 * t8798 - t1336 * t33839 + t544 * t33841;
    (t33839, t33841, t33843)
}
