//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1305/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1305<F: Float>(t1014: F, t27928: F, t26717: F, t8030: F, t27808: F, t27911: F, t27964: F, t7693: F, t7703: F, t92943: F, t93542: F, t93562: F, t95985: F, t95989: F, t95992: F, t96000: F, t96003: F) -> (F, F) {
    let t96005 = t1014 * t27928;
    let t96010 = F::cast_from(0.46336805555555555556e-3_f64) * t8030 * t26717;
    let t96011 = F::cast_from(0.30891203703703703704e-3_f64) * t7703 * t95985 + F::cast_from(0.88437037037037037034e-2_f64) * t95989 + F::cast_from(0.66327777777777777776e-2_f64) * t95992 - F::cast_from(0.22109259259259259258e-2_f64) * t92943 + F::cast_from(0.41188271604938271606e-3_f64) * t93542 + F::cast_from(0.4946917361111111111e-3_f64) * t93562 * t27911 + F::cast_from(0.14840752083333333333e-2_f64) * t93562 * t27808 + F::cast_from(0.55273148148148148147e-3_f64) * t96000 + F::cast_from(0.16581944444444444444e-2_f64) * t96003 + F::cast_from(0.22109259259259259258e-2_f64) * t96005 - F::cast_from(0.37069444444444444444e-2_f64) * t27964 * t7693 + t96010;
    (t96005, t96011)
}
