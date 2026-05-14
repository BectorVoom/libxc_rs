//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 674/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk674<F: Float>(t4531: F, t458: F, t4527: F, t15885: F, t24: F, t469: F, t15917: F, t1787: F, t15752: F, t3134: F, t11668: F, t11669: F, t11684: F, t11686: F, t16370: F, t16373: F, t16375: F, t16378: F, t16381: F, t16384: F, t16387: F, t16392: F, t16396: F, t16401: F, t3139: F, t462: F, t8283: F, t92: F) -> (F,) {
    let t16404 = t458 * t4531;
    let t16406 = t458 * t4527;
    let t16409 = t24 * t469 * t15885;
    let t16411 = t1787 * t15917;
    let t16414 = t3134 * t15752;
    let t16417 = t11668 - 8.0 / 9.0 * t11669 - 4.0 / 27.0 * t8283 + t462 * t16370 / 3.0 - 2.0 / 9.0 * t16373 - 2.0 / 3.0 * t462 * t16375 + t462 * t16378 / 3.0 + 2.0 / 3.0 * t462 * t16381 - 2.0 / 9.0 * t462 * t16384 + 4.0 / 3.0 * t3139 * t16387 + 2.0 * t462 * t16392 - t462 * t16396 / 3.0 - 6.0 * t462 * t16401 - t11684 + t11686 + t16404 / 3.0 - 2.0 / 3.0 * t16406 - t92 * t16409 - 2.0 / 3.0 * t462 * t16411 - 2.0 * t462 * t16414;
    (t16417,)
}
