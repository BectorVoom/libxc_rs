//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 893/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk893<F: Float>(t1035: F, t18857: F, t1072: F, t6307: F, t331: F, t6313: F, t1027: F, t6317: F, t6353: F, t829: F, t6272: F, t1045: F) -> (F, F, F, F, F, F, F) {
    let t19327 = t1035 * t18857;
    let t19330 = t1072 * t6307;
    let t19332 = t331 * t6313;
    let t19334 = t1027 * t6317;
    let t19336 = t1027 * t6353;
    let t19340 = t6307 * t829;
    let t19343 = t1035 * t6272;
    let t19344 = t19343 * t1045;
    (t19327, t19330, t19332, t19334, t19336, t19340, t19344)
}
