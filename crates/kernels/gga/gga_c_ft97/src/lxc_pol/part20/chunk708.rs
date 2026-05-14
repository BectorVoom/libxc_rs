//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 708/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk708<F: Float>(t14690: F, t15229: F, t4311: F, t684: F, t835: F, t2867: F, t4246: F, t840: F, t3746: F, t882: F, t15138: F, t296: F, t1212: F, t2894: F, t10461: F, t10463: F, t15202: F, t15206: F, t15208: F, t15212: F, t15218: F, t15222: F, t15226: F, t1901: F, t3281: F, t446: F) -> (F,) {
    let t15230 = t15229 * t14690;
    let t15234 = t835 * t4311 * t684;
    let t15238 = t840 * t4246 * t2867;
    let t15242 = t835 * t882 * t3746;
    let t15245 = t296 * t15138;
    let t15249 = t840 * t2894 * t1212;
    let t15252 = -2.0 / 9.0 * t1901 * t15202 + t15206 + 2.0 / 3.0 * t446 * t15208 - t446 * t15212 / 3.0 - 2.0 / 27.0 * t10461 - 2.0 / 27.0 * t10463 - 2.0 / 3.0 * t446 * t15218 - t446 * t15222 / 3.0 - 2.0 / 9.0 * t1901 * t15226 - 4.0 / 9.0 * t1901 * t15230 - 2.0 / 9.0 * t446 * t15234 + 2.0 / 3.0 * t446 * t15238 + 4.0 / 9.0 * t3281 * t15242 - t446 * t15245 / 3.0 - t446 * t15249 / 3.0;
    (t15252,)
}
