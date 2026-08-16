//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3280/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3280<F: Float>(t39760: F, t39764: F, t39770: F, t39773: F, t39779: F, t39783: F, t39786: F, t39791: F, t39795: F, t61149: F, t61150: F, t61151: F, t61159: F, t61161: F, t61162: F, t61166: F, t61167: F, t61168: F, t61169: F) -> F {
    let t62262 = t39760 - t39764 + t61149 + t39770 - t61150 + t61151 + t61159 + t39773 + t61161 - t61162 + t61166 - t61167 + t39779 - t61168 - t61169 - t39783 - t39786 - t39791 - t39795;
    t62262
}
