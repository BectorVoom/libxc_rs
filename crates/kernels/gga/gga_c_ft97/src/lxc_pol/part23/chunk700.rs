//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 700/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk700<F: Float>(t17780: F, t3917: F, t17722: F, t2493: F, t18303: F, t18305: F, t18308: F, t18312: F, t18314: F, t18316: F, t18318: F, t18321: F, t18324: F, t18327: F, t18330: F, t18333: F, t18336: F, t18339: F, t18342: F, t18345: F, t18348: F, t18351: F, t3139: F, t462: F, t92: F) -> (F,) {
    let t18354 = t3917 * t17780;
    let t18357 = t2493 * t17722;
    let t18360 = 2.0 / 27.0 * t18303 - 2.0 / 9.0 * t18305 - t462 * t18308 / 3.0 - t92 * t18312 + t18314 / 3.0 - 2.0 / 3.0 * t18316 + 2.0 / 3.0 * t462 * t18318 - 2.0 / 9.0 * t462 * t18321 + t462 * t18324 / 3.0 + 2.0 / 9.0 * t462 * t18327 + 4.0 / 3.0 * t462 * t18330 - 10.0 / 27.0 * t462 * t18333 + 8.0 / 9.0 * t3139 * t18336 + 2.0 / 3.0 * t462 * t18339 - 4.0 / 3.0 * t3139 * t18342 - 2.0 / 3.0 * t462 * t18345 - 2.0 / 3.0 * t462 * t18348 - 2.0 * t462 * t18351 - 8.0 / 3.0 * t3139 * t18354 + t462 * t18357 / 3.0;
    (t18360,)
}
