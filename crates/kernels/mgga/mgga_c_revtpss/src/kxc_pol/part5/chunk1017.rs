//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1017/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1017<F: Float>(t2986: F, t960: F, t11132: F, t1034: F, t3154: F, t357: F, t1024: F, t3105: F, t905: F, t606: F, t1052: F, t360: F) -> (F, F, F, F, F, F, F, F) {
    let t11554 = t960 * t2986;
    let t11560 = F::cast_from(0.28842592592592592592e-1_f64) * t11132;
    let t11574 = F::cast_from(0.53272592592592592592e-1_f64) * t11132;
    let t11626 = t1034 * t1034;
    let t11627 = F::new(1.0) / t11626;
    let t11631 = t3154 * t357;
    let t11656 = t1024 * t3105;
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    let t11670 = t360 * t1052;
    (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670)
}
