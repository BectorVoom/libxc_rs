//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 908/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk908<F: Float>(t1587: F, t27: F, t85687: F, t89: F, t16474: F, t4533: F, t91: F, t20388: F, t3119: F, t38392: F, t57620: F, t73343: F, t73358: F, t73405: F, t86246: F, t86250: F, t86254: F, t86258: F, t86264: F, t86268: F, t86274: F) -> (F, F, F, F) {
    let t86278 = t89 * t27 * t1587 * t85687;
    let t86281 = t91 * t16474 * t4533;
    let t86284 = t91 * t3119 * t20388;
    let t86285 = 40.0 / 9.0 * t86246 + 8.0 * t86250 - 80.0 / 81.0 * t86254 - t86258 / 3.0 - 8.0 * t73343 + t38392 - 4.0 / 3.0 * t73358 + 8.0 * t86264 + 2.0 * t86268 + 16.0 / 3.0 * t57620 - 8.0 / 9.0 * t73405 + 24.0 * t86274 + 6.0 * t86278 + 9.0 / 4.0 * t86281 - t86284;
    (t86278, t86281, t86284, t86285)
}
