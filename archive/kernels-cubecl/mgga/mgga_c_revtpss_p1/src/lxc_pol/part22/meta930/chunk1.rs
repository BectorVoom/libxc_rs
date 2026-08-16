//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3158/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3158<F: Float>(t12916: F, t17747: F, t17749: F, t1222: F, t16725: F, t17471: F, t16729: F, t13017: F, t5373: F, t44546: F, t5331: F, t5334: F) -> (F, F, F, F, F) {
    let t57191 = t17747 * t12916 * t17749;
    let t57209 = t1222 * t17471 * t16725;
    let t57212 = t1222 * t17471 * t16729;
    let t57214 = t5373 * t13017;
    let t57222 = t5331 * t44546 * t5334;
    (t57191, t57209, t57212, t57214, t57222)
}
