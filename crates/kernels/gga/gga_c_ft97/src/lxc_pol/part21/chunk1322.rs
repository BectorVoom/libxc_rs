//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1322/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1322<F: Float>(t1882: F, t30536: F, t1053: F, t106871: F, t106906: F, t106912: F, t106914: F, t106928: F, t107141: F, t107316: F, t119468: F, t13220: F, t1391: F, t144: F, t16666: F, t17071: F, t17076: F, t17081: F, t17496: F, t1901: F, t2185: F, t23997: F, t26768: F, t3424: F, t3429: F, t446: F, t47659: F, t574: F, t5935: F, t605: F, t9144: F, t95842: F) -> (F,) {
    let t121162 = t1882 * t30536;
    let t121193 = -t106871 + 2.0 / 3.0 * t446 * t574 * t605 * t26768 * t1053 - 2.0 / 9.0 * t121162 - 2.0 / 3.0 * t446 * t574 * t23997 * t17081 - 2.0 / 3.0 * t446 * t2185 * t5935 * t17071 + 4.0 / 3.0 * t446 * t2185 * t1391 * t17076 + 4.0 / 3.0 * t47659 * t107316 * t16666 + 4.0 / 9.0 * t47659 * t95842 * t17496 + 4.0 / 3.0 * t446 * t144 * t119468 - t106906 - 2.0 / 9.0 * t1901 * t9144 * t107141 * t3424 - 4.0 / 9.0 * t1901 * t13220 * t107141 * t3429 + t106912 - t106914 + t106928;
    (t121193,)
}
