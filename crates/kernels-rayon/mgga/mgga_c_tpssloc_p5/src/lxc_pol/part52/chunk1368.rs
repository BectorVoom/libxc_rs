//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1368/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1368(t131: f64, t2240: f64, t27331: f64, t46104: f64, t8662: f64, t12571: f64, t31867: f64, t33676: f64, t9231: f64, t27363: f64, t8301: f64, t116106: f64, t116115: f64, t119888: f64, t119897: f64, t119965: f64, t121040: f64, t121044: f64, t121050: f64, t121055: f64, t121099: f64, t121102: f64, t121105: f64, t121108: f64, t122941: f64, t31019: f64, t31684: f64, t31857: f64, t31864: f64, t31868: f64, t33115: f64, t33669: f64, t33677: f64, t8515: f64, t8663: f64) -> f64 {
    let t122945 = t2240 * t27331 * t131;
    let t122952 = t46104 * t8662;
    let t122955 = t12571 * t31867;
    let t122960 = t9231 * t33676;
    let t122964 = t2240 * t8301 * t27363;
    let t122975 = 5.0_f64 / 6.0_f64 * t116106 * t121108 + 5.0_f64 / 6.0_f64 * t116106 * t121105 - 5.0_f64 / 18.0_f64 * t31864 * t121102 - 5.0_f64 / 18.0_f64 * t31864 * t121099 - 5.0_f64 / 36.0_f64 * t31864 * t119888 - 5.0_f64 / 36.0_f64 * t31864 * t121040 - 5.0_f64 / 36.0_f64 * t31864 * t121044 + 5.0_f64 / 18.0_f64 * t122941 * t121050 - 5.0_f64 / 36.0_f64 * t122945 * t31684 - 5.0_f64 / 12.0_f64 * t116115 * t121055 - 5.0_f64 / 36.0_f64 * t31864 * t119897 + 5.0_f64 / 144.0_f64 * t122952 * t8515 + 5.0_f64 / 144.0_f64 * t122955 * t8515 + 5.0_f64 / 144.0_f64 * t33669 * t31019 + 5.0_f64 / 144.0_f64 * t122960 * t8515 + 5.0_f64 / 144.0_f64 * t122964 * t8515 + 5.0_f64 / 144.0_f64 * t33677 * t31019 + 5.0_f64 / 144.0_f64 * t31857 * t33115 + 5.0_f64 / 144.0_f64 * t31868 * t33115 + 5.0_f64 / 144.0_f64 * t8663 * t119965;
    t122975
}
