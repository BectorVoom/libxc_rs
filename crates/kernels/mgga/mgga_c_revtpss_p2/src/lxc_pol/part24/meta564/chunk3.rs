//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1705/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1705<F: Float>(t1042: F, t15707: F, t19697: F, t23635: F, t23643: F, t23823: F, t23834: F, t42745: F, t42920: F, t42921: F, t4879: F, t53704: F, t53707: F, t6302: F, t66547: F, t79546: F, t79548: F, t79553: F, t79564: F, t79575: F, t79580: F, t88695: F) -> F {
    let t89202 = -F::cast_from(0.51448821741683684368e-2_f64) * t53704 * t23834 + F::cast_from(0.85748036236139473944e-3_f64) * t53707 * t23643 + F::cast_from(0.34299214494455789577e-2_f64) * t15707 * t23635 - F::cast_from(0.21437009059034868486e-3_f64) * t42920 * t1042 * t88695 * t42921 + F::cast_from(0.12862205435420921092e-2_f64) * t19697 * t6302 + F::cast_from(0.85748036236139473944e-3_f64) * t4879 * t23823 + F::cast_from(0.11433071498151929859e-2_f64) * t79546 + F::cast_from(0.11433071498151929859e-2_f64) * t79548 - F::cast_from(0.22866142996303859718e-2_f64) * t79553 + F::cast_from(0.34299214494455789578e-2_f64) * t79564 - t42745 - F::cast_from(0.34299214494455789578e-2_f64) * t79575 - F::cast_from(0.22866142996303859718e-2_f64) * t79580 + t66547 / F::cast_from(108.0_f64);
    t89202
}
