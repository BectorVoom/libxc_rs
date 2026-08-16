//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1347/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1347(t100137: f64, t100189: f64, t100193: f64, t100231: f64, t106028: f64, t1610: f64, t21510: f64, t21643: f64, t23327: f64, t23601: f64, t23633: f64, t23677: f64, t23678: f64, t25470: f64, t25510: f64, t25511: f64, t28617: f64, t28634: f64, t28642: f64, t3200: f64, t3201: f64, t4669: f64, t5866: f64, t5903: f64, t7603: f64, t7619: f64, t7622: f64) -> f64 {
    let t106113 = 0.16449340668482264365e-1_f64 * t23327 * t25470 * t28617 + 3.0_f64 * t1610 * t28634 + 3.0_f64 * t5903 * t7622 + 0.82246703342411321826e-2_f64 * t23633 * t100231 * t106028 - 0.16449340668482264365e-1_f64 * t23327 * t25510 * t25511 * t21510 + 0.49348022005446793095e-1_f64 * t23601 * t23677 * t21643 * t23678 - 0.82246703342411321826e-2_f64 * t23327 * t100137 * t7603 - 3.0_f64 * t3200 * t7619 * t3201 * t5866 + 0.36554090374405031922e-2_f64 * t100189 + 3.0_f64 * t4669 * t28642 - 0.16449340668482264365e-1_f64 * t100193;
    t106113
}
