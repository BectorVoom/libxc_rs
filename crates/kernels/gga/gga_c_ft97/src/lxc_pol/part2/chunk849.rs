//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 849/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk849<F: Float>(t10461: F, t10463: F, t15202: F, t15206: F, t15208: F, t15212: F, t15218: F, t15222: F, t15226: F, t15230: F, t15234: F, t15238: F, t15242: F, t15245: F, t15249: F, t1901: F, t3281: F, t446: F) -> (F,) {
    let t15252 = -2.0 / 9.0 * t1901 * t15202 + t15206 + 2.0 / 3.0 * t446 * t15208 - t446 * t15212 / 3.0 - 2.0 / 27.0 * t10461 - 2.0 / 27.0 * t10463 - 2.0 / 3.0 * t446 * t15218 - t446 * t15222 / 3.0 - 2.0 / 9.0 * t1901 * t15226 - 4.0 / 9.0 * t1901 * t15230 - 2.0 / 9.0 * t446 * t15234 + 2.0 / 3.0 * t446 * t15238 + 4.0 / 9.0 * t3281 * t15242 - t446 * t15245 / 3.0 - t446 * t15249 / 3.0;
    (t15252,)
}
