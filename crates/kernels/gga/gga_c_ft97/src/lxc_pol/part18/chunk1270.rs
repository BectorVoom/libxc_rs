//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1270/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1270<F: Float>(t27435: F, t5: F, t1080: F, t12253: F, t12261: F, t13260: F, t13263: F, t13268: F, t13276: F, t13279: F, t2309: F, t24157: F, t27440: F, t3660: F, t3665: F, t3668: F, t5985: F, t650: F, t96310: F) -> (F,) {
    let t104095 = t5 * t27435;
    let t104104 = t24157 * t3665 / 2.0 + t5985 * t12261 / 4.0 + t5985 * t13263 / 4.0 + t5985 * t12253 / 2.0 + t24157 * t3660 / 2.0 + t24157 * t3668 / 2.0 - 3.0 / 2.0 * t5985 * t13276 + t5985 * t13268 / 4.0 + t5985 * t13260 / 4.0 + t104095 * t650 / 2.0 + t5985 * t13279 / 4.0 + t96310 * t1080 / 4.0 + t27440 * t2309 / 2.0;
    (t104104,)
}
