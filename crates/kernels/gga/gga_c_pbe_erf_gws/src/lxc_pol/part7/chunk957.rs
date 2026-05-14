//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 957/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk957<F: Float>(t19278: F, t5803: F, t5813: F, t119: F, t155: F, t5645: F, t1513: F, t1533: F, t299: F, t799: F, t5795: F, t481: F, t837: F, t1365: F, t497: F, t496: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19279 = t5803 * t19278;
    let t19281 = t5813 * t19278;
    let t19282 = 0.77947333333333333333e1 * t19281;
    let t19284 = t119 * t155 * t5645;
    let t19285 = t1513 * t19284;
    let t19286 = 0.19486833333333333333e1 * t19285;
    let t19288 = t799 * t299 * t1533;
    let t19289 = t5795 * t19288;
    let t19290 = 0.19486833333333333333e1 * t19289;
    let t19292 = t119 * t837 * t481;
    let t19293 = t1513 * t19292;
    let t19294 = 0.60625703703703703703e1 * t19293;
    let t19295 = t1365 * t497;
    let t19296 = t496 * t19295;
    (t19279, t19282, t19284, t19286, t19288, t19290, t19292, t19294, t19295, t19296)
}
