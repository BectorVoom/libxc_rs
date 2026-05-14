//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 804/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk804<F: Float>(t13306: F, t13308: F, t13329: F, t13335: F, t13338: F, t13339: F, t13345: F, t13388: F, t13680: F, t13682: F, t13688: F, t18271: F, t18276: F, t18279: F, t18283: F, t18286: F, t18290: F, t18295: F, t18299: F, t462: F, t9907: F, t9935: F, t9936: F) -> (F,) {
    let t18302 = -t13306 + t13308 - t13329 - 8.0 / 27.0 * t13335 - t13338 - 4.0 / 9.0 * t13339 - 4.0 / 9.0 * t9936 + t13345 + 4.0 / 9.0 * t13682 * t18271 - 4.0 / 3.0 * t13688 * t18276 - 4.0 / 3.0 * t13688 * t18279 - 4.0 / 27.0 * t9907 - 2.0 / 9.0 * t18283 - t9935 - t13388 - 8.0 / 9.0 * t13680 + t18286 / 9.0 + 2.0 * t462 * t18290 - 6.0 * t462 * t18295 + 4.0 * t462 * t18299;
    (t18302,)
}
