//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 913/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk913<F: Float>(t242: F, t27899: F, t14200: F, t27763: F, t14163: F, t27767: F, t684: F, t6861: F, t10007: F, t1882: F, t6927: F, t11593: F, t1901: F, t24590: F, t24592: F, t28150: F, t28154: F, t28158: F, t28163: F, t28167: F, t28171: F, t28175: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t28178 = t242 * t27899;
    let t28181 = t14200 * t27763;
    let t28184 = t14163 * t27767;
    let t28187 = t6861 * t684;
    let t28188 = t10007 * t28187;
    let t28191 = t1882 * t6927;
    let t28193 = t1901 * t28150 / 9.0 + t1901 * t28154 / 9.0 - 2.0 / 9.0 * t11593 * t28158 - 2.0 / 9.0 * t24590 - t24592 / 9.0 - 2.0 / 9.0 * t1901 * t28163 - t446 * t28167 / 3.0 - t446 * t28171 / 3.0 - t446 * t28175 / 3.0 - t446 * t28178 / 3.0 + 2.0 / 27.0 * t1901 * t28181 - 2.0 / 9.0 * t1901 * t28184 - t1901 * t28188 / 9.0 + t28191 / 9.0;
    (t28178, t28181, t28184, t28187, t28188, t28191, t28193)
}
