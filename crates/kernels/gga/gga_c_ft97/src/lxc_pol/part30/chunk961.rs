//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 961/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk961<F: Float>(t28735: F, t28736: F, t33868: F, t840: F, t143042: F, t143112: F, t28496: F, t33820: F, t143040: F, t143041: F, t28511: F, t143158: F, t152669: F, t10683: F, t6317: F, t6318: F) -> (F, F, F, F, F) {
    let t152694 = t28735 * t840 * t33868 * t28736;
    let t152698 = t33820 * t143112 * t143042 * t28496;
    let t152702 = t143040 * t143041 * t143042 * t28511;
    let t152704 = t33820 * t143158 * t152669;
    let t152708 = t6317 * t10683 * t6318 * t28496;
    (t152694, t152698, t152702, t152704, t152708)
}
