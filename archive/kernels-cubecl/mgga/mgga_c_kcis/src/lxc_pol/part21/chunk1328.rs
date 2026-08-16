//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1328/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1328<F: Float>(t27936: F, t7699: F, t13420: F, t3200: F, t92808: F, t26742: F, t8042: F, t93728: F, t93742: F, t93750: F, t96399: F, t96402: F, t96404: F, t96407: F, t96410: F, t96412: F) -> (F, F) {
    let t96418 = F::cast_from(0.46336805555555555556e-3_f64) * t27936 * t7699;
    let t96420 = t3200 * t92808 * t13420;
    let t96424 = -F::cast_from(0.58958024691358024689e-2_f64) * t96399 - t96402 + F::cast_from(0.33163888888888888888e-2_f64) * t96404 - F::cast_from(0.33163888888888888888e-2_f64) * t96407 + F::cast_from(0.22109259259259259258e-2_f64) * t96410 - F::cast_from(0.3684876543209876543e-3_f64) * t96412 + F::cast_from(0.67960648148148148147e-2_f64) * t26742 * t8042 - F::cast_from(0.46336805555555555556e-3_f64) * t93728 - t96418 - F::cast_from(0.22109259259259259258e-2_f64) * t96420 + F::cast_from(0.12356481481481481482e-2_f64) * t93742 - F::cast_from(0.22653549382716049383e-2_f64) * t93750;
    (t96420, t96424)
}
