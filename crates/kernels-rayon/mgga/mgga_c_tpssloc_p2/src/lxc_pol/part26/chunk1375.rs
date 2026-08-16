//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1375/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1375(t11172: f64, t11616: f64, t1244: f64, t1246: f64, t2152: f64, t24667: f64, t24841: f64, t24849: f64, t24852: f64, t3493: f64, t3604: f64, t3611: f64, t3624: f64, t3625: f64, t470: f64, t493: f64, t7283: f64, t7348: f64, t7362: f64, t7363: f64, t86032: f64, t86037: f64, t86095: f64, t86102: f64, t86106: f64, t86113: f64, t86116: f64, t86376: f64) -> f64 {
    let t86381 = -3.0_f64 * t3624 * t86032 * t3625 - 0.54831135561607547883e-2_f64 * t86095 + 3.0_f64 * t1244 * t7348 * t3493 * t1246 + 0.82246703342411321826e-2_f64 * t86037 * t24667 * t3611 * t86102 + 0.36554090374405031922e-2_f64 * t86106 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t7363 * t11172 + t11616 * t2152 - 0.82246703342411321826e-2_f64 * t86113 - 0.16449340668482264365e-1_f64 * t24849 * t86116 * t24852 + t470 * t493 * t86376 + 6.0_f64 * t3604 * t24841;
    t86381
}
