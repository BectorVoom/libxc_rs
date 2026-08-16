//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1705/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1705(t1042: f64, t15707: f64, t19697: f64, t23635: f64, t23643: f64, t23823: f64, t23834: f64, t42745: f64, t42920: f64, t42921: f64, t4879: f64, t53704: f64, t53707: f64, t6302: f64, t66547: f64, t79546: f64, t79548: f64, t79553: f64, t79564: f64, t79575: f64, t79580: f64, t88695: f64) -> f64 {
    let t89202 = -0.51448821741683684368e-2_f64 * t53704 * t23834 + 0.85748036236139473944e-3_f64 * t53707 * t23643 + 0.34299214494455789577e-2_f64 * t15707 * t23635 - 0.21437009059034868486e-3_f64 * t42920 * t1042 * t88695 * t42921 + 0.12862205435420921092e-2_f64 * t19697 * t6302 + 0.85748036236139473944e-3_f64 * t4879 * t23823 + 0.11433071498151929859e-2_f64 * t79546 + 0.11433071498151929859e-2_f64 * t79548 - 0.22866142996303859718e-2_f64 * t79553 + 0.34299214494455789578e-2_f64 * t79564 - t42745 - 0.34299214494455789578e-2_f64 * t79575 - 0.22866142996303859718e-2_f64 * t79580 + t66547 / 108.0_f64;
    t89202
}
