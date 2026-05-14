//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 572/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk572<F: Float>(t8299: F, t8335: F, t457: F, t91: F, t369: F, t631: F, t637: F, t7242: F, t96: F, t1767: F, t473: F, t1766: F, t1808: F, t7754: F, t7771: F, t7782: F, t7786: F, t7804: F, t7820: F, t8186: F, t8192: F, t8195: F, t8260: F) -> (F, F, F, F, F, F) {
    let t8336 = t8299 + t8335;
    let t8338 = t91 * t457 * t8336;
    let t8345 = 1.0 / t96 / t631 / t637 / t369 / t7242 / 4.0;
    let t8346 = t1767 * t473;
    let t8348 = t91 * t8345 * t8346;
    let t8352 = t91 * t1766 * t473 * t1808;
    let t8354 = 2.0 / 9.0 * t7782 - 2.0 * t7786 + 4.0 / 3.0 * t7804 - 2.0 / 3.0 * t7820 - t8186 - 4.0 / 3.0 * t8192 + t8195 - 6.0 * t7754 - 2.0 * t7771 - t8260 + t8338 / 2.0 + 3.0 / 8.0 * t8348 - 3.0 / 4.0 * t8352;
    (t8336, t8338, t8345, t8348, t8352, t8354)
}
