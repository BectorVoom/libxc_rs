//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1227/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1227<F: Float>(t3145: F, t334: F, t368: F, t3153: F, t73: F, t246: F, t676: F, t1046: F, t1041: F, t3140: F, t989: F, t3149: F) -> (F, F, F, F, F, F) {
    let t11243 = F::cast_from(1.0_f64) / t3145 / t368 / t334;
    let t11249 = t3153 * t73;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11273 = t989 * t3140;
    let t11274 = t11273 * t3149;
    (t11243, t11249, t11262, t11264, t11273, t11274)
}
