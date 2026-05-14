//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1169/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1169<F: Float>(t26742: F, t8042: F, t93728: F, t93742: F, t93750: F, t96399: F, t96402: F, t96404: F, t96407: F, t96410: F, t96412: F, t96418: F, t96420: F, t1014: F, t27879: F, t303: F, t4923: F, t7731: F) -> (F, F, F, F) {
    let t96424 = -0.58958024691358024689e-2 * t96399 - t96402 + 0.33163888888888888888e-2 * t96404 - 0.33163888888888888888e-2 * t96407 + 0.22109259259259259258e-2 * t96410 - 0.3684876543209876543e-3 * t96412 + 0.67960648148148148147e-2 * t26742 * t8042 - 0.46336805555555555556e-3 * t93728 - t96418 - 0.22109259259259259258e-2 * t96420 + 0.12356481481481481482e-2 * t93742 - 0.22653549382716049383e-2 * t93750;
    let t96427 = t1014 * t27879;
    let t96428 = 0.33163888888888888888e-2 * t96427;
    let t96430 = t303 * t4923 * t7731;
    (t96424, t96427, t96428, t96430)
}
