//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 878/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk878<F: Float>(t1045: F, t3133: F, t373: F, t1042: F, t1031: F, t196: F) -> (F, F, F) {
    let t3135 = t373 * t3133 * t1045;
    let t3136 = t1042 * t3135;
    let t3140 = F::cast_from(1.0_f64) / t1031 / t196;
    (t3135, t3136, t3140)
}
