//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 276/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk276<F: Float>(t442: F, t967: F, t222: F, t441: F) -> (F, F, F, F) {
    let t1387 = t967 * t442;
    let t1388 = F::cast_from(0.5179538907796306876e-4_f64) * t1387;
    let t1389 = t441 * t222;
    let t1390 = F::cast_from(1.0_f64) / t1389;
    (t1387, t1388, t1389, t1390)
}
