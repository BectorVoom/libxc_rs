//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 605/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk605<F: Float>(t3319: F, t3320: F, t783: F, t1060: F, t560: F, t1058: F, t2201: F, t481: F, t2207: F, t1059: F, t269: F) -> (F, F, F, F, F, F) {
    let t3322 = t783 * t3319 * t3320;
    let t3323 = F::cast_from(0.23287303101564395623e-1_f64) * t3322;
    let t3324 = t1060 * t560;
    let t3326 = t2201 * t1058 * t3324;
    let t3328 = t1060 * t481;
    let t3330 = t2207 * t1058 * t3328;
    let t3332 = t269 * t1059;
    (t3323, t3324, t3326, t3328, t3330, t3332)
}
