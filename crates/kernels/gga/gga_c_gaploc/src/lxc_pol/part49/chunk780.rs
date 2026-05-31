//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 780/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk780<F: Float>(t13225: F, t943: F, t2592: F, t3511: F, t1052: F, t9767: F, t3263: F, t5559: F, t977: F, t1960: F, t3322: F, t3459: F, t7324: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13226 = t943 * t13225;
    let t13232 = t2592 * t3511;
    let t13234 = t9767 * t1052;
    let t13235 = t1052 * t3263;
    let t13237 = F::cast_from(6.0_f64) * t5559 * t13235;
    let t13238 = t3511 * t977;
    let t13239 = t1960 * t13238;
    let t13241 = t1052 * t3322;
    let t13243 = F::cast_from(2.0_f64) * t1960 * t13241;
    let t13245 = F::cast_from(4.0_f64) * t7324 * t3459;
    (t13226, t13232, t13234, t13235, t13237, t13238, t13239, t13241, t13243, t13245)
}
