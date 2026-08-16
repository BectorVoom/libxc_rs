//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3186/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3186<F: Float>(t12910: F, t12916: F, t17624: F, t17709: F, t17712: F, t3766: F, t5219: F, t5330: F, t17601: F, t3718: F, t12855: F, t17579: F) -> (F, F, F, F, F) {
    let t59149 = t12910 * t12916 * t17624;
    let t59159 = t17709 * t12916 * t17712;
    let t59162 = t5219 * t3766 * t5330;
    let t59173 = t3718 * t12916 * t17601;
    let t59176 = t12855 * t12916 * t17579;
    (t59149, t59159, t59162, t59173, t59176)
}
