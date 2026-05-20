//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2759/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2759<F: Float>(t10587: F, t2516: F, t2401: F, t2609: F, t2519: F, t268: F, t9306: F) -> (F, F, F) {
    let t39774 = t10587 * t2516;
    let t39779 = t2401 * t2609;
    let t39783 = F::cast_from(0.71233333333333333332e-1_f64) * t268 * t2519 * t9306;
    (t39774, t39779, t39783)
}
