//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 940/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk940<F: Float>(t2851: F, t749: F, t8191: F, t1765: F, t2942: F, t1070: F, t1775: F, t1067: F, t1799: F, t8197: F, t8199: F, t8204: F) -> (F, F, F, F, F, F, F, F) {
    let t11327 = t2851 * t749;
    let t11328 = F::cast_from(144.0_f64) * t11327;
    let t11333 = F::cast_from(48.0_f64) * t8191;
    let t11335 = t1765 * t2942;
    let t11337 = t1070 * t1775;
    let t11338 = F::cast_from(96.0_f64) * t11337;
    let t11339 = t1067 * t1799;
    let t11340 = F::cast_from(36.0_f64) * t11339;
    let t11341 = F::cast_from(480.0_f64) * t8197;
    let t11342 = F::cast_from(144.0_f64) * t8199;
    let t11343 = F::cast_from(240.0_f64) * t8204;
    (t11328, t11333, t11335, t11338, t11340, t11341, t11342, t11343)
}
