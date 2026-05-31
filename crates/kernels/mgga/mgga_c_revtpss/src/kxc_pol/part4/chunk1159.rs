//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1159/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1159<F: Float>(t2619: F, t4302: F, t4186: F, t750: F, t706: F, t4395: F, t10556: F, t4537: F, t892: F, t123: F, t1534: F, t2630: F) -> (F, F, F, F, F, F) {
    let t14339 = t4302 * t2619;
    let t14340 = F::cast_from(0.24415263074675393405e-3_f64) * t14339;
    let t14341 = t750 * t4186;
    let t14343 = F::cast_from(8.0_f64) * t706 * t14341;
    let t14345 = F::cast_from(2.0_f64) * t4395 * t750;
    let t14352 = F::cast_from(12.0_f64) * t10556;
    let t14353 = t4537 * t892;
    let t14362 = t1534 * t123;
    let t14363 = t14362 * t2630;
    (t14340, t14343, t14345, t14352, t14353, t14363)
}
