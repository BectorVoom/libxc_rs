//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 722/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk722<F: Float>(t1390: F, t561: F, t828: F, t1955: F, t239: F, t8571: F, t555: F, t8477: F) -> (F, F, F) {
    let t8575 = t1390 * t828 * t561;
    let t8576 = t1955 * t8571 * t239 * t8575;
    let t8583 = t8477 * t555;
    (t8575, t8576, t8583)
}
