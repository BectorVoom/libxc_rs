//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1034/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1034(t115508: f64, t122187: f64, t122192: f64, t122204: f64, t122210: f64, t122213: f64, t122218: f64, t16460: f64, t24082: f64, t26996: f64, t27068: f64, t27132: f64, t32151: f64, t32176: f64, t33798: f64, t3882: f64, t5215: f64, t5321: f64, t7194: f64, t7199: f64, t7925: f64, t8801: f64) -> f64 {
    let t124093 = 2.0_f64 * t3882 * t33798 + 2.0_f64 * t5215 * t32176 + 4.0_f64 * t7194 * t27132 + 4.0_f64 * t7194 * t26996 + 0.6579736267392905746e-1_f64 * t122187 - 0.15352717957250113407e0_f64 * t115508 - 0.3289868133696452873e-1_f64 * t122192 + 4.0_f64 * t27068 * t7199 + 0.6579736267392905746e-1_f64 * t122204 + 4.0_f64 * t24082 * t7925 - t5215 * t32151 - t16460 * t8801 + 0.76763589786250567037e-1_f64 * t122210 + 0.6579736267392905746e-1_f64 * t122213 - 0.13159472534785811492e0_f64 * t122218 + 2.0_f64 * t5321 * t32176;
    t124093
}
