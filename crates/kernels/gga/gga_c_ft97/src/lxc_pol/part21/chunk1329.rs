//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1329/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1329<F: Float>(t1882: F, t30465: F, t107243: F, t107273: F, t107294: F, t107296: F, t107303: F, t107323: F, t107336: F, t12680: F, t17022: F, t17376: F, t17380: F, t1901: F, t2221: F, t23581: F, t27245: F, t27263: F, t3578: F, t4458: F, t446: F, t47659: F, t47666: F, t574: F, t5943: F, t63258: F, t95751: F, t95975: F) -> (F,) {
    let t121410 = t1882 * t30465;
    let t121432 = -t107243 - 4.0 / 81.0 * t95975 + t107294 + t107296 - 2.0 / 9.0 * t1901 * t2221 * t23581 * t4458 - t107303 + t121410 / 9.0 - 2.0 / 9.0 * t1901 * t95751 * t17022 + t1901 * t63258 * t5943 / 9.0 - 4.0 / 27.0 * t107323 - t107336 + 2.0 / 9.0 * t1901 * t12680 * t27245 + 8.0 / 9.0 * t47659 * t107273 * t17376 - 8.0 / 27.0 * t47666 * t107273 * t17380 + 2.0 / 3.0 * t446 * t574 * t3578 * t27263;
    (t121432,)
}
