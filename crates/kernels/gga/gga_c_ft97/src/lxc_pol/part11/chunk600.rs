//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 600/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk600<F: Float>(t8299: F, t8335: F, t457: F, t91: F, t369: F, t631: F, t637: F, t7242: F, t96: F, t1767: F, t473: F, t1766: F, t1808: F) -> (F, F, F, F, F) {
    let t8336 = t8299 + t8335;
    let t8338 = t91 * t457 * t8336;
    let t8345 = F::new(1.0) / t96 / t631 / t637 / t369 / t7242 / F::new(4.0);
    let t8346 = t1767 * t473;
    let t8348 = t91 * t8345 * t8346;
    let t8352 = t91 * t1766 * t473 * t1808;
    (t8336, t8338, t8345, t8348, t8352)
}
