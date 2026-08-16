//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3163/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163<F: Float>(t12916: F, t17780: F, t5331: F, t1260: F, t45385: F, t12640: F, t17728: F, t489: F, t12744: F, t17350: F, t3781: F, t5219: F, t5330: F) -> (F, F, F, F, F) {
    let t57336 = t5331 * t12916 * t17780;
    let t57344 = t45385 * t1260;
    let t57348 = t12640 * t489 * t17728;
    let t57378 = t12744 * t17350;
    let t57382 = t5219 * t3781 * t5330;
    (t57336, t57344, t57348, t57378, t57382)
}
