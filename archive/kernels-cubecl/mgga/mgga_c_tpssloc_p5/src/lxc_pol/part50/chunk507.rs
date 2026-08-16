//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 507/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk507<F: Float>(t3236: F, t407: F, t281: F, t2820: F, t415: F, t1114: F, t699: F, t1176: F, t241: F, t1097: F, t409: F, t422: F) -> (F, F, F, F, F, F, F, F) {
    let t3282 = F::cast_from(0.39862222222222222223e0_f64) * t3236;
    let t3287 = F::cast_from(1.0_f64)/F::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    let t3294 = F::cast_from(0.13692777777777777778e0_f64) * t3293;
    let t3295 = t699 * t1114;
    let t3297 = t241 * t1176;
    let t3311 = t1097 * t1097;
    let t3312 = F::cast_from(1.0_f64) / t3311;
    let t3313 = t409 * t3312;
    let t3314 = t422 * t422;
    (t3282, t3287, t3293, t3294, t3295, t3297, t3313, t3314)
}
