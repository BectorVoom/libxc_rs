//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 108/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk108<F: Float>(t397: F, t399: F, t403: F, t396: F, t139: F, t201: F, t79: F) -> (F, F, F, F, F) {
    let t405 = t397 * t399 * t403;
    let t408 = F::new(1.0) + F::cast_from(0.5397236614853195164e-1_f64) * t396 * t405;
    let t409 = F::ln(t408);
    let t411 = F::new(1.0) + F::new(0.193e0) * t409;
    let t412 = F::new(1.0) / t411;
    let t415 = t139 * t201 * t79;
    (t405, t408, t411, t412, t415)
}
