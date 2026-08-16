//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1287/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1287(t225: f64, t34254: f64, t8872: f64, t94490: f64, t24574: f64, t34244: f64, t118084: f64, t1252: f64, t125510: f64, t15797: f64, t1761: f64, t24589: f64, t24615: f64, t27388: f64, t27406: f64, t27742: f64, t27746: f64, t27747: f64, t27751: f64, t27760: f64, t27784: f64, t27785: f64, t32510: f64, t32516: f64, t32519: f64, t34306: f64, t34318: f64, t34331: f64, t34338: f64, t3487: f64, t3593: f64, t466: f64, t498: f64, t7283: f64, t7300: f64, t7351: f64, t86403: f64, t8888: f64) -> f64 {
    let t125713 = t34254 * t225;
    let t125729 = t94490 * t8872;
    let t125732 = t24574 * t34244;
    let t125752 = -t125713 * t1252 - 0.54831135561607547883e-2_f64 * t7283 * t32519 * t27388 + 0.3289868133696452873e-1_f64 * t7283 * t27751 * t32510 + 4.0_f64 * t7351 * t27747 - 12.0_f64 * t27784 * t27785 * t27760 - t3593 * t34306 + 2.0_f64 * t15797 * t8888 + 0.14621636149762012769e-1_f64 * t125729 - t118084 * t1761 - 0.54831135561607547883e-2_f64 * t125732 - 2.0_f64 * t7351 * t27742 + t466 * t125510 * t498 - 0.54831135561607547883e-2_f64 * t24589 * t86403 * t34338 - 0.43864908449286038307e-1_f64 * t27406 * t32516 + 0.3289868133696452873e-1_f64 * t7283 * t7300 * t24615 * t27746 - t3487 * t34306 - 6.0_f64 * t3593 * t34331 + 2.0_f64 * t3593 * t34318;
    t125752
}
