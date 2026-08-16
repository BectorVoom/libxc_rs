//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2948/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948<F: Float>(t43401: F, t53668: F, t11247: F, t15758: F, t15787: F, t15910: F, t15957: F, t15963: F, t16084: F, t3091: F, t3092: F, t3117: F, t3154: F, t357: F, t42369: F, t42374: F, t42377: F, t42383: F, t53654: F, t53657: F, t53661: F, t53669: F, t53670: F) -> F {
    let t53676 = t43401 * t53668;
    let t53682 = -F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t15957 * t15963 + F::cast_from(0.12862205435420921092e-2_f64) * t15758 * t15787 + F::cast_from(0.38586616306262763275e-2_f64) * t53654 * t16084 - F::cast_from(0.38586616306262763275e-2_f64) * t53657 * t15910 + F::cast_from(0.17149607247227894789e-2_f64) * t53661 + F::cast_from(0.85748036236139473944e-3_f64) * t42369 - F::cast_from(0.42874018118069736972e-3_f64) * t42374 - F::cast_from(0.85748036236139473944e-3_f64) * t42377 - F::cast_from(0.22866142996303859718e-2_f64) * t42383 + F::cast_from(0.30011812682648815881e-2_f64) * t53669 * t3117 * t53670 * t11247 * t3154 - F::cast_from(0.21437009059034868486e-3_f64) * t53676 * t3117 * t53670 * t11247 * t357;
    t53682
}
