//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 928/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk928<F: Float>(t1061: F, t31891: F, t31892: F, t3143: F, t3268: F, t1039: F, t31997: F, t1045: F, t1096: F, t25638: F, t7150: F, t120179: F, t3089: F, t31973: F, t120190: F, t8514: F) -> (F, F, F, F, F, F) {
    let t120263 = t31891 * t31892 * t1061;
    let t120273 = t3268 * t3143;
    let t120275 = t31997 * t120273 * t1039;
    let t120276 = t1045 * t1096;
    let t120281 = t7150 * t25638;
    let t120285 = t31973 * t120179 * t3089;
    let t120288 = t8514 * t120190;
    (t120263, t120275, t120276, t120281, t120285, t120288)
}
