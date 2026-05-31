//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 482/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk482<F: Float>(t1034: F, t358: F, t368: F, t335: F, t73: F, t357: F, t1038: F, t1052: F, t1036: F, t1033: F, t127: F, t246: F) -> (F, F, F, F, F, F) {
    let t3143 = F::cast_from(1.0_f64) / t1034 / t358;
    let t3145 = t368 * t368;
    let t3147 = F::cast_from(1.0_f64) / t3145 / t335;
    let t3153 = t73 * t73;
    let t3154 = t357 * t357;
    let t3167 = t1052 * t1038;
    let t3168 = t1036 * t3167;
    let t3169 = t1033 * t3168;
    let t3172 = t246 * t127;
    (t3143, t3147, t3153, t3154, t3169, t3172)
}
