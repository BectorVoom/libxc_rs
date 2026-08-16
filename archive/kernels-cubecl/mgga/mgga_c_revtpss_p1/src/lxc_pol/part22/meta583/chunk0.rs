//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2446/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2446<F: Float>(t18657: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F) -> (F, F, F, F) {
    let t18658 = t18657 * t225;
    let t18662 = t6048 * t886;
    let t18663 = t11008 * t18662;
    let t18677 = t251 * t5977;
    (t18658, t18662, t18663, t18677)
}
