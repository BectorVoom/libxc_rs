//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1116/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1116<F: Float>(t10772: F, t10810: F, t2578: F, t1577: F, t2599: F, t3308: F, t574: F, t7527: F, t2096: F, t2649: F, t571: F, t10769: F) -> (F, F, F, F) {
    let t39400 = t10772 * t10810 * t2578;
    let t39401 = F::cast_from(0.69345773920434148506e0_f64) * t39400;
    let t39403 = t1577 * t10810 * t2599;
    let t39404 = F::cast_from(0.46230515946956099004e0_f64) * t39403;
    let t39406 = t574 * t3308 * t7527;
    let t39409 = t571 * t2649 * t2096;
    let t39410 = t39409 * t10769;
    (t39401, t39404, t39406, t39410)
}
