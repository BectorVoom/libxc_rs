//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 777/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk777<F: Float>(t1238: F, t413: F, t10471: F, t1278: F, t3668: F, t1280: F, t433: F) -> (F, F, F, F, F, F, F, F) {
    let t11181 = t1238 * t1238;
    let t11182 = F::new(1.0) / t11181;
    let t11183 = t413 * t11182;
    let t11209 = F::new(0.51588271604938271604e-3) * t10471;
    let t11223 = t1278 * t3668;
    let t11228 = t1280 * t1280;
    let t11229 = F::new(1.0) / t11228;
    let t11230 = t433 * t11229;
    (t11181, t11182, t11183, t11209, t11223, t11228, t11229, t11230)
}
