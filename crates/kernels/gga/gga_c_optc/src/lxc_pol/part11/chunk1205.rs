//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1205/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1205<F: Float>(t1102: F, t26214: F, t26217: F, t58311: F, t15146: F, t18183: F, t1220: F, t1221: F, t4297: F, t43571: F, t53279: F, t53281: F, t53290: F, t53293: F, t53299: F, t58295: F, t58308: F, t58310: F, t914: F) -> (F, F) {
    let t58315 = 0.91080982599109921211e5 * t1102 * t26214 * t58311 * t26217;
    let t58316 = t15146 * t18183;
    let t58319 = t1220 * t914 * t1221 * t58295 / 6.0 - 64.0 / 27.0 * t53279 - 32.0 / 9.0 * t53281 - 400.0 / 81.0 * t53290 - 4.0 / 3.0 * t53293 - 200.0 / 9.0 * t53299 + 200.0 / 9.0 * t43571 + t58308 - t58310 - t58315 + 800.0 / 81.0 * t4297 * t58316;
    (t58315, t58319)
}
