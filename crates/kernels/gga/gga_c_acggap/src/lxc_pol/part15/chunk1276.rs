//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1276/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1276<F: Float>(t10761: F, t11179: F, t19418: F, t2249: F, t2408: F, t24794: F, t24811: F, t36769: F, t36771: F, t36774: F, t38534: F, t38540: F, t40955: F, t5399: F, t5439: F, t567: F, t643: F, t6614: F, t7297: F, t8040: F, t8372: F, t9096: F, t9460: F, t9476: F) -> F {
    let t42366 = -F::new(6.0) * t10761 * t7297 * t9476 - F::new(6.0) * t11179 * t5439 * t7297 - t19418 * t567 * t643 - t2249 * t567 * t6614 - F::new(2.0) * t2408 * t5399 * t567 + F::new(2.0) * t24794 * t9096 * t9460 - F::new(3.0) * t24811 * t7297 * t8040 + F::new(6.0) * t38534 * t7297 * t9460 + F::new(4.0) * t38540 * t9096 * t9460 - F::new(12.0) * t40955 * t8040 * t8372 - t36769 + t36771 + t36774;
    t42366
}
