//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1070/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1070<F: Float>(t19288: F, t5795: F, t119: F, t481: F, t837: F, t1513: F, t1365: F, t497: F, t496: F, t1548: F, t542: F, t156: F, t5790: F) -> (F, F, F, F, F, F, F, F) {
    let t19289 = t5795 * t19288;
    let t19290 = F::cast_from(0.19486833333333333333e1_f64) * t19289;
    let t19292 = t119 * t837 * t481;
    let t19293 = t1513 * t19292;
    let t19294 = F::cast_from(0.60625703703703703703e1_f64) * t19293;
    let t19295 = t1365 * t497;
    let t19296 = t496 * t19295;
    let t19298 = t542 * t1548;
    let t19299 = t496 * t19298;
    let t19301 = t156 * t5790;
    (t19290, t19292, t19294, t19295, t19296, t19298, t19299, t19301)
}
