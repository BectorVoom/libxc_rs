//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 883/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk883(t1375: f64, t1843: f64, t2016: f64, t2092: f64, t26366: f64, t26477: f64, t27009: f64, t31129: f64, t31571: f64, t31653: f64, t32696: f64, t32764: f64, t32771: f64, t33241: f64, t33247: f64, t33251: f64, t33269: f64, t33274: f64, t33294: f64, t33298: f64, t33301: f64, t33332: f64, t5215: f64, t5321: f64, t6958: f64, t7194: f64, t7750: f64, t7937: f64, t8637: f64) -> f64 {
    let t33334 = -0.82246703342411321825e-2_f64 * t33241 - t31653 * t1843 + 0.82246703342411321825e-2_f64 * t33247 + 0.16449340668482264365e-1_f64 * t33251 - t7194 * t7750 + t32696 - t5215 * t8637 - t31571 - t26366 * t2092 - t26477 * t2092 + t33269 + t31129 + t32764 - t27009 * t2016 + 0.16449340668482264365e-1_f64 * t33274 - t1375 * t33294 - 0.82246703342411321825e-2_f64 * t33298 - t32771 + 2.0_f64 * t1375 * t33301 - t5321 * t8637 - t6958 * t7937 + t33332;
    t33334
}
