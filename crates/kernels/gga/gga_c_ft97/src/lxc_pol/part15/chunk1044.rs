//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1044/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1044<F: Float>(t5309: F, t72231: F, t1248: F, t84519: F, t15195: F, t15290: F, t15299: F, t1901: F, t19571: F, t22376: F, t2881: F, t296: F, t446: F, t4973: F, t71522: F, t71532: F, t71534: F, t71589: F, t84080: F, t84087: F, t89805: F, t89809: F) -> (F, F, F) {
    let t90632 = t72231 * t5309;
    let t90652 = t84519 * t1248;
    let t90664 = 2.0 / 3.0 * t1901 * t2881 * t19571 * t4973 - 16.0 / 9.0 * t71522 - 8.0 / 3.0 * t1901 * t15299 * t89805 + 16.0 / 9.0 * t71532 + 16.0 / 9.0 * t71534 + 4.0 / 3.0 * t84080 - 4.0 / 3.0 * t446 * t296 * t90652 + 4.0 / 3.0 * t84087 + 8.0 / 9.0 * t1901 * t15290 * t89809 - 16.0 / 9.0 * t71589 - 8.0 / 3.0 * t1901 * t15195 * t22376;
    (t90632, t90652, t90664)
}
