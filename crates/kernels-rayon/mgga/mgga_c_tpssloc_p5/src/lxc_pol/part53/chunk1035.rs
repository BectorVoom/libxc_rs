//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1035/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1035(t40590: f64, t8793: f64, t115530: f64, t117284: f64, t122227: f64, t122235: f64, t122247: f64, t1375: f64, t16030: f64, t16439: f64, t2091: f64, t2092: f64, t26224: f64, t27114: f64, t32161: f64, t33798: f64, t33804: f64, t33810: f64, t3758: f64, t3887: f64, t5215: f64, t5321: f64, t5325: f64, t8801: f64, t93316: f64) -> f64 {
    let t124103 = t40590 * t8793;
    let t124122 = -6.0_f64 * t3758 * t33810 - 0.19739208802178717238e0_f64 * t122227 - t16030 * t8801 + 4.0_f64 * t3758 * t33804 - 0.3289868133696452873e-1_f64 * t122235 + 24.0_f64 * t26224 * t124103 * t5325 + 4.0_f64 * t1375 * t3887 * t2091 * t27114 - 2.0_f64 * t93316 * t2092 - 0.76763589786250567037e-1_f64 * t115530 - 6.0_f64 * t5321 * t32161 - t16439 * t8801 - t117284 + 0.16449340668482264365e-1_f64 * t122247 + 2.0_f64 * t3758 * t33798 - 6.0_f64 * t5215 * t32161;
    t124122
}
