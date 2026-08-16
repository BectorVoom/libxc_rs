//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1018/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1018(t116065: f64, t117447: f64, t625: f64, t79: f64, t641: f64, t8513: f64, t8663: f64, t113824: f64, t113864: f64, t113876: f64, t116075: f64, t116082: f64, t116106: f64, t116124: f64, t117451: f64, t117461: f64, t2241: f64, t2244: f64, t2307: f64, t31857: f64, t31860: f64, t31864: f64, t31868: f64, t32328: f64, t32340: f64, t7246: f64, t8824: f64) -> f64 {
    let t117477 = t116065 * t117447;
    let t117480 = t79 * t625;
    let t117483 = t8663 * t8513 * t117480 * t641;
    let t117487 = -10.0_f64 / 3.0_f64 * t116106 * t117447 * t113864 + 10.0_f64 / 9.0_f64 * t31864 * t117451 * t113876 - 35.0_f64 / 12.0_f64 * t116075 * t8513 * t8824 * t2241 - 20.0_f64 / 9.0_f64 * t117461 + 5.0_f64 / 18.0_f64 * t7246 * t8513 * t8824 * t2244 + 5.0_f64 / 6.0_f64 * t116124 * t32328 - 5.0_f64 / 18.0_f64 * t31857 * t32340 + 5.0_f64 / 6.0_f64 * t116082 * t32328 + 5.0_f64 / 12.0_f64 * t31860 * t8513 * t8824 * t2307 - 5.0_f64 / 9.0_f64 * t113824 * t117477 + 20.0_f64 / 27.0_f64 * t117483 - 5.0_f64 / 18.0_f64 * t31868 * t32340;
    t117487
}
