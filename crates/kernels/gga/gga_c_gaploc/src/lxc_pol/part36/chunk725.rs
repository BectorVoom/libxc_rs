//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 725/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk725<F: Float>(t2592: F, t3511: F, t1052: F, t9767: F, t3263: F, t5559: F, t977: F, t1960: F, t3322: F, t3459: F, t7324: F, t12845: F, t12850: F, t12851: F, t12861: F, t12864: F) -> (F, F, F, F) {
    let t13232 = t2592 * t3511;
    let t13234 = t9767 * t1052;
    let t13235 = t1052 * t3263;
    let t13237 = F::new(6.0) * t5559 * t13235;
    let t13238 = t3511 * t977;
    let t13239 = t1960 * t13238;
    let t13241 = t1052 * t3322;
    let t13243 = F::new(2.0) * t1960 * t13241;
    let t13245 = F::new(4.0) * t7324 * t3459;
    let t13246 = t12851 - F::new(2.0) * t13232 - t13234 - t12845 - t13237 - t12861 - t12864 + F::new(4.0) * t13239 + t13243 + t13245 + t12850;
    (t13235, t13238, t13241, t13246)
}
