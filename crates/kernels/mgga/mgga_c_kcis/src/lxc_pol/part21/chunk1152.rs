//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1152/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1152<F: Float>(t1662: F, t93426: F, t93427: F, t1009: F, t14400: F, t14633: F, t3200: F, t92808: F, t8048: F, t9562: F, t1014: F, t27925: F, t1748: F, t3225: F, t303: F, t3229: F) -> (F, F, F, F, F, F, F) {
    let t96105 = t93426 * t1662 * t93427;
    let t96108 = t14400 * t1009;
    let t96116 = t3200 * t92808 * t14633;
    let t96121 = t9562 * t8048;
    let t96123 = t1014 * t27925;
    let t96124 = 0.33163888888888888888e-2 * t96123;
    let t96125 = t1748 * t3225;
    let t96127 = t303 * t96125 * t3229;
    (t96105, t96108, t96116, t96121, t96123, t96124, t96127)
}
