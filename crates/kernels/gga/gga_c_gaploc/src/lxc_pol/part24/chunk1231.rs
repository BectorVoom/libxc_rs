//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1231/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1231<F: Float>(t204: F, t2476: F, t34567: F, t34621: F, t34623: F, t34626: F, t34628: F, t34631: F, t34634: F, t34636: F, t34638: F, t34640: F, t34643: F, t34645: F, t34648: F, t34650: F, t34652: F) -> (F,) {
    let t34656 = t34621 + t34623 + t34626 + t34628 + t34631 + t34634 + t34636 + t34638 - t34640 - t34643 - t34645 + t34648 + t34650 + t34652 + 0.92023022289409799224e1 * t2476 * t204 * t34567;
    (t34656,)
}
