//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 995/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk995<F: Float>(t33820: F, t33822: F, t37041: F, t4162: F, t898: F, t143042: F, t143112: F, t28501: F, t143040: F, t28776: F, t33821: F, t3628: F, t3746: F, t6307: F, t143041: F, t28816: F) -> (F, F, F, F, F) {
    let t153414 = t33820 * t898 * t37041 * t33822 * t4162;
    let t153418 = t33820 * t143112 * t143042 * t28501;
    let t153422 = t143040 * t143112 * t33822 * t28776;
    let t153427 = t6307 * t3628 * t33821 * t33822 * t3746;
    let t153431 = t143040 * t143041 * t143042 * t28816;
    (t153414, t153418, t153422, t153427, t153431)
}
