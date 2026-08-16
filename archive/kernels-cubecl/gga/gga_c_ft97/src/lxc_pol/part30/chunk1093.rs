//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1093/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1093<F: Float>(t143040: F, t143158: F, t152661: F, t24980: F, t24981: F, t35853: F, t684: F, t35833: F, t24976: F, t6317: F, t143217: F, t3886: F) -> (F, F, F, F, F) {
    let t152663 = t143040 * t143158 * t152661;
    let t152667 = t24980 * t24981 * t35853 * t684;
    let t152669 = t35833 * t684;
    let t152671 = t6317 * t24976 * t152669;
    let t152673 = t143217 * t3886;
    (t152663, t152667, t152669, t152671, t152673)
}
