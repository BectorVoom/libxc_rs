//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2494/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2494<F: Float>(t15421: F, t4636: F, t6110: F, t934: F, t2924: F, t1610: F, t4631: F, t2874: F, t6145: F, t11299: F, t6142: F, t2926: F, t6141: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19317 = F::cast_from(0.32163958997385070134e2_f64) * t15421 * t4636;
    let t19318 = t6110 * t934;
    let t19320 = F::cast_from(6.0_f64) * t2924 * t19318;
    let t19321 = t1610 * t4631;
    let t19323 = F::cast_from(4.0_f64) * t2874 * t19321;
    let t19324 = t6145 * t934;
    let t19326 = F::cast_from(0.96491876992155210402e2_f64) * t11299 * t19324;
    let t19327 = t6142 * t934;
    let t19329 = F::cast_from(2.0_f64) * t2874 * t19327;
    let t19330 = t6141 * t2926;
    (t19317, t19318, t19320, t19321, t19323, t19324, t19326, t19327, t19329, t19330)
}
