//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1368/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1368<F: Float>(t131: F, t2240: F, t27331: F, t46104: F, t8662: F, t12571: F, t31867: F, t33676: F, t9231: F, t27363: F, t8301: F, t116106: F, t116115: F, t119888: F, t119897: F, t119965: F, t121040: F, t121044: F, t121050: F, t121055: F, t121099: F, t121102: F, t121105: F, t121108: F, t122941: F, t31019: F, t31684: F, t31857: F, t31864: F, t31868: F, t33115: F, t33669: F, t33677: F, t8515: F, t8663: F) -> F {
    let t122945 = t2240 * t27331 * t131;
    let t122952 = t46104 * t8662;
    let t122955 = t12571 * t31867;
    let t122960 = t9231 * t33676;
    let t122964 = t2240 * t8301 * t27363;
    let t122975 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t116106 * t121108 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t116106 * t121105 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t121102 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t121099 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31864 * t119888 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31864 * t121040 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31864 * t121044 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t122941 * t121050 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t122945 * t31684 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t116115 * t121055 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31864 * t119897 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t122952 * t8515 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t122955 * t8515 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t33669 * t31019 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t122960 * t8515 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t122964 * t8515 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t33677 * t31019 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t31857 * t33115 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t31868 * t33115 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t8663 * t119965;
    t122975
}
