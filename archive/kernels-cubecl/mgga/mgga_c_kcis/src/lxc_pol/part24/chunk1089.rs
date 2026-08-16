//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1089/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1089<F: Float>(t236: F, t28300: F, t233: F, t27836: F, t8047: F, t1020: F, t3203: F, t6276: F, t7718: F, t4555: F, t6272: F, t2842: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28301 = t236 * t28300;
    let t28302 = t233 * t28301;
    let t28904 = t27836 * t8047;
    let t28905 = t1020 * t28904;
    let t28907 = t3203 * t6276;
    let t28908 = t7718 * t28907;
    let t28909 = t1020 * t28908;
    let t28911 = t4555 * t6272;
    let t28912 = t7718 * t28911;
    let t28913 = t2842 * t28912;
    (t28302, t28904, t28905, t28907, t28908, t28909, t28911, t28912, t28913)
}
