//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1094/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1094<F: Float>(t143158: F, t152673: F, t33820: F, t143239: F, t3886: F, t2917: F, t7641: F, t143041: F, t28736: F, t33822: F, t631: F, t99475: F) -> (F, F, F, F) {
    let t152675 = t33820 * t143158 * t152673;
    let t152678 = t143239 * t3886;
    let t152680 = t33820 * t2917 * t7641 * t152678;
    let t152686 = t99475 * t631 * t143041 * t33822 * t28736;
    (t152675, t152678, t152680, t152686)
}
