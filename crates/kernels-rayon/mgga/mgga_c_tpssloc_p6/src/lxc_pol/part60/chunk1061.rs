//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1061/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1061(t127560: f64, t127562: f64, t128908: f64, t128909: f64, t128922: f64, t128924: f64, t128926: f64, t129008: f64, t130442: f64, t1774: f64, t2040: f64, t28002: f64, t32350: f64, t33690: f64, t34137: f64, t34150: f64, t34170: f64, t4028: f64, t510: f64, t5494: f64, t6287: f64, t6468: f64, t7787: f64, t7796: f64, t8103: f64, t8829: f64, t8835: f64, t8840: f64) -> f64 {
    let t130492 = -2.0_f64 * t129008 * t2040 - t130442 * t510 - 2.0_f64 * t1774 * t34137 - 4.0_f64 * t28002 * t8835 - 2.0_f64 * t32350 * t5494 - 4.0_f64 * t33690 * t7796 - 4.0_f64 * t34150 * t4028 - 4.0_f64 * t34170 * t4028 - t6287 * t8829 + t6468 * t8840 - 2.0_f64 * t7787 * t8103 - t127560 - t127562 + t128908 + t128909 - t128922 - t128924 + t128926;
    t130492
}
