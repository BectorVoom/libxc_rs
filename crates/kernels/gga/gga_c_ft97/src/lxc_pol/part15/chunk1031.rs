//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1031/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1031<F: Float>(t356: F, t7801: F, t85469: F, t89: F, t1571: F, t85451: F, t27: F, t37429: F, t85692: F, t1587: F, t85687: F, t16474: F, t4533: F, t91: F) -> (F, F, F, F, F) {
    let t86264 = t89 * t356 * t7801 * t85469;
    let t86268 = t89 * t356 * t1571 * t85451;
    let t86274 = t89 * t27 * t37429 * t85692;
    let t86278 = t89 * t27 * t1587 * t85687;
    let t86281 = t91 * t16474 * t4533;
    (t86264, t86268, t86274, t86278, t86281)
}
