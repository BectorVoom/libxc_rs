//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 673/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk673<F: Float>(t4965: F, t735: F, t1527: F, t1751: F, t1411: F, t378: F, t1398: F, t468: F, t1510: F, t410: F, t4911: F, t89: F) -> (F, F, F, F, F, F) {
    let t4966 = t735 * t4965;
    let t4967 = F::new(0.48159733137676571078e0) * t4966;
    let t4968 = t1751 * t1527;
    let t4970 = t378 * t1411;
    let t4971 = t735 * t4970;
    let t4972 = F::new(0.16265371950452609763e-1) * t4971;
    let t4973 = t1398 * t468;
    let t4974 = t735 * t4973;
    let t4975 = F::new(0.21687162600603479684e-1) * t4974;
    let t4978 = t410 * t1510;
    let t4979 = F::new(12.0) * t4978;
    let t4980 = t4911 * t89;
    (t4967, t4968, t4972, t4975, t4979, t4980)
}
