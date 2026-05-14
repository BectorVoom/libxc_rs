//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 722/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk722<F: Float>(t1716: F, t2541: F, t1949: F, t945: F, t1933: F, t78: F, t278: F, t481: F) -> (F, F, F) {
    let t7204 = t2541 * t1716;
    let t7207 = t1949 * t945;
    let t7209 = t78 * t1933;
    let t7211 = t481 * t7209 * t278;
    (t7204, t7207, t7211)
}
