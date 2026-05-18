//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 849/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk849<F: Float>(t22216: F, t22258: F, t22396: F, t22467: F, t22439: F, t312: F, t1218: F, t1253: F, t21931: F, t21933: F, t22168: F, t22250: F, t22347: F, t22356: F, t22360: F, t22406: F, t22464: F, t301: F, t317: F, t5207: F, t5305: F, t5422: F) -> (F, F, F) {
    let t22469 = t22216 + t22258 + t22396 + t22467;
    let t22471 = t22439 * t312;
    let t22479 = -F::new(3.0) * t1218 * t5422 - F::new(3.0) * t1253 * t5207 - F::new(3.0) * t1253 * t5305 - t21931 * t317 - F::new(2.0) * t21933 * t317 - t22168 * t317 - t22469 * t301 - F::new(12.0) * t22250 - F::new(2.0) * t22347 + F::new(12.0) * t22356 - F::new(6.0) * t22360 + F::new(12.0) * t22406 - F::new(6.0) * t22464 + F::new(2.0) * t22471;
    (t22469, t22471, t22479)
}
