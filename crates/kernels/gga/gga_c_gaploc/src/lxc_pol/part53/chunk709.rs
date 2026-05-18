//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 709/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk709<F: Float>(t13235: F, t5559: F, t1052: F, t3322: F, t1960: F, t3459: F, t7324: F, t2321: F, t3701: F, t882: F, t11981: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t13237 = F::new(6.0) * t5559 * t13235;
    let t13241 = t1052 * t3322;
    let t13243 = F::new(2.0) * t1960 * t13241;
    let t13245 = F::new(4.0) * t7324 * t3459;
    let t13725 = t3701 * t2321;
    let t13726 = t882 * t13725;
    let t13728 = t11981 * t874;
    (t13237, t13241, t13243, t13245, t13725, t13726, t13728)
}
