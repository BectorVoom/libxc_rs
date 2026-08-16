//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1305/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1305(t1014: f64, t27928: f64, t26717: f64, t8030: f64, t27808: f64, t27911: f64, t27964: f64, t7693: f64, t7703: f64, t92943: f64, t93542: f64, t93562: f64, t95985: f64, t95989: f64, t95992: f64, t96000: f64, t96003: f64) -> (f64, f64) {
    let t96005 = t1014 * t27928;
    let t96010 = 0.46336805555555555556e-3_f64 * t8030 * t26717;
    let t96011 = 0.30891203703703703704e-3_f64 * t7703 * t95985 + 0.88437037037037037034e-2_f64 * t95989 + 0.66327777777777777776e-2_f64 * t95992 - 0.22109259259259259258e-2_f64 * t92943 + 0.41188271604938271606e-3_f64 * t93542 + 0.4946917361111111111e-3_f64 * t93562 * t27911 + 0.14840752083333333333e-2_f64 * t93562 * t27808 + 0.55273148148148148147e-3_f64 * t96000 + 0.16581944444444444444e-2_f64 * t96003 + 0.22109259259259259258e-2_f64 * t96005 - 0.37069444444444444444e-2_f64 * t27964 * t7693 + t96010;
    (t96005, t96011)
}
