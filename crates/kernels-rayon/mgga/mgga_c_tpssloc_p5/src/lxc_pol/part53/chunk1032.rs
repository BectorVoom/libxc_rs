//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1032/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1032(t225: f64, t33823: f64, t115339: f64, t115341: f64, t115354: f64, t117133: f64, t122145: f64, t122150: f64, t122152: f64, t122160: f64, t122164: f64, t1386: f64, t16030: f64, t16460: f64, t2092: f64, t27009: f64, t27068: f64, t33844: f64, t3758: f64, t7214: f64, t8794: f64, t93341: f64) -> f64 {
    let t124019 = t33823 * t225;
    let t124040 = -t117133 - t124019 * t1386 - t3758 * t33844 + 0.15352717957250113407e0_f64 * t115339 + 0.76763589786250567037e-1_f64 * t115341 + 0.6579736267392905746e-1_f64 * t122145 - 2.0_f64 * t93341 * t2092 + 2.0_f64 * t16460 * t8794 + 0.6579736267392905746e-1_f64 * t122150 - 0.76763589786250567037e-1_f64 * t122152 - 2.0_f64 * t27009 * t7214 - 2.0_f64 * t27068 * t7214 + 2.0_f64 * t16030 * t8794 + 0.3289868133696452873e-1_f64 * t115354 + 0.3289868133696452873e-1_f64 * t122160 - 0.3289868133696452873e-1_f64 * t122164;
    t124040
}
