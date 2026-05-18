//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 900/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk900<F: Float>(t1029: F, t3228: F, t166: F, t1: F, t1035: F, t1039: F, t3669: F, t3036: F, t3213: F, t996: F, t117: F, t3033: F) -> (F, F, F, F, F) {
    let t13459 = t3228 * t1029;
    let t13461 = t166 * t166;
    let t13462 = F::new(1.0) / t13461;
    let t13463 = t13462 * t1;
    let t13474 = F::new(0.68026775414003982664e-1) * t1035 * t3669 * t1039;
    let t13481 = F::new(0.24009450146119052705e-1) * t3036 * t996 * t3213;
    let t13483 = F::new(1.0) / t3033 / t117;
    (t13459, t13463, t13474, t13481, t13483)
}
