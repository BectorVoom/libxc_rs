//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1130/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1130<F: Float>(t125388: F, t28219: F, t8460: F, t32322: F, t7901: F, t32392: F, t7742: F, t32394: F, t28063: F, t8634: F, t28184: F, t8568: F) -> (F, F, F, F, F, F, F) {
    let t125389 = F::cast_from(2.0_f64) * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = F::cast_from(2.0_f64) * t125390;
    let t125402 = t32322 * t7901;
    let t125405 = F::cast_from(4.0_f64) * t32392 * t7742;
    let t125407 = F::cast_from(4.0_f64) * t32394 * t7742;
    let t125409 = F::cast_from(4.0_f64) * t8634 * t28063;
    let t125410 = t8568 * t28184;
    (t125389, t125391, t125402, t125405, t125407, t125409, t125410)
}
