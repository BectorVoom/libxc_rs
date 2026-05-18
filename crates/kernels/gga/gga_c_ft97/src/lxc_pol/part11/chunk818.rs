//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 818/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk818<F: Float>(t122: F, t695: F, t677: F, t25: F, t200: F, t709: F, t807: F, t9542: F, t9524: F, t173: F, t2440: F, t420: F, t9651: F) -> (F, F, F, F, F, F, F) {
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13521 = t200 * t709;
    let t13531 = t807 * t9542;
    let t13589 = t9524 * t9542;
    let t13598 = t173 * t2440;
    let t13605 = t420 * t9651;
    (t13468, t13474, t13521, t13531, t13589, t13598, t13605)
}
