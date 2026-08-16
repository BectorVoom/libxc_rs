//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1328/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1328(t27936: f64, t7699: f64, t13420: f64, t3200: f64, t92808: f64, t26742: f64, t8042: f64, t93728: f64, t93742: f64, t93750: f64, t96399: f64, t96402: f64, t96404: f64, t96407: f64, t96410: f64, t96412: f64) -> (f64, f64) {
    let t96418 = 0.46336805555555555556e-3_f64 * t27936 * t7699;
    let t96420 = t3200 * t92808 * t13420;
    let t96424 = -0.58958024691358024689e-2_f64 * t96399 - t96402 + 0.33163888888888888888e-2_f64 * t96404 - 0.33163888888888888888e-2_f64 * t96407 + 0.22109259259259259258e-2_f64 * t96410 - 0.3684876543209876543e-3_f64 * t96412 + 0.67960648148148148147e-2_f64 * t26742 * t8042 - 0.46336805555555555556e-3_f64 * t93728 - t96418 - 0.22109259259259259258e-2_f64 * t96420 + 0.12356481481481481482e-2_f64 * t93742 - 0.22653549382716049383e-2_f64 * t93750;
    (t96420, t96424)
}
