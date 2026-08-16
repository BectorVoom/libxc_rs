//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2948/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948(t43401: f64, t53668: f64, t11247: f64, t15758: f64, t15787: f64, t15910: f64, t15957: f64, t15963: f64, t16084: f64, t3091: f64, t3092: f64, t3117: f64, t3154: f64, t357: f64, t42369: f64, t42374: f64, t42377: f64, t42383: f64, t53654: f64, t53657: f64, t53661: f64, t53669: f64, t53670: f64) -> f64 {
    let t53676 = t43401 * t53668;
    let t53682 = -0.85748036236139473944e-3_f64 * t3091 * t3092 * t15957 * t15963 + 0.12862205435420921092e-2_f64 * t15758 * t15787 + 0.38586616306262763275e-2_f64 * t53654 * t16084 - 0.38586616306262763275e-2_f64 * t53657 * t15910 + 0.17149607247227894789e-2_f64 * t53661 + 0.85748036236139473944e-3_f64 * t42369 - 0.42874018118069736972e-3_f64 * t42374 - 0.85748036236139473944e-3_f64 * t42377 - 0.22866142996303859718e-2_f64 * t42383 + 0.30011812682648815881e-2_f64 * t53669 * t3117 * t53670 * t11247 * t3154 - 0.21437009059034868486e-3_f64 * t53676 * t3117 * t53670 * t11247 * t357;
    t53682
}
