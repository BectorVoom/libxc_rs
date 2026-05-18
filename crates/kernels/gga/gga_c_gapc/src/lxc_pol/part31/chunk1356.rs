//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1356/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1356<F: Float>(t11043: F, t3449: F, t10544: F, t8601: F, t31754: F, t3268: F, t2468: F, t3828: F, t2470: F, t10086: F, t3565: F, t10529: F, t8616: F) -> (F, F, F, F, F, F) {
    let t36316 = F::new(2.0) * t11043 * t3449;
    let t36318 = F::new(2.0) * t8601 * t10544;
    let t36320 = F::new(4.0) * t31754 * t3268;
    let t36321 = t3828 * t2468;
    let t36323 = F::new(2.0) * t36321 * t2470;
    let t36324 = t3565 * t10086;
    let t36326 = F::new(2.0) * t10529 * t8616;
    (t36316, t36318, t36320, t36323, t36324, t36326)
}
