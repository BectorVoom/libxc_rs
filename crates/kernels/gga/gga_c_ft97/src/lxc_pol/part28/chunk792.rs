//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 792/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk792<F: Float>(t144: F, t35038: F, t574: F, t5935: F, t6699: F, t569: F, t7414: F, t925: F, t1053: F, t7312: F, t2185: F, t605: F, t167: F, t34822: F, t3578: F, t7357: F) -> (F, F, F, F, F, F, F) {
    let t35039 = t144 * t35038;
    let t35043 = t574 * t5935 * t6699;
    let t35047 = t569 * t7414 * t925;
    let t35050 = t7312 * t1053;
    let t35052 = t2185 * t605 * t35050;
    let t35056 = t2185 * t167 * t34822;
    let t35060 = t574 * t3578 * t7357;
    (t35039, t35043, t35047, t35050, t35052, t35056, t35060)
}
