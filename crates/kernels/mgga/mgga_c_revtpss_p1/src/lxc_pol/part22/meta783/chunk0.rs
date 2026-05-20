//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2873/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2873<F: Float>(t17847: F, t3588: F, t17854: F, t1209: F, t17887: F, t12657: F, t3754: F, t12722: F, t3555: F, t12640: F, t3552: F, t3766: F) -> (F, F, F, F, F, F, F) {
    let t45675 = t17847 * t3588;
    let t45679 = t17854 * t3588;
    let t45683 = t1209 * t17887;
    let t45697 = t12657 * t3754;
    let t45700 = t3555 * t12722;
    let t45707 = t12640 * t3754;
    let t45710 = t3552 * t3766;
    (t45675, t45679, t45683, t45697, t45700, t45707, t45710)
}
