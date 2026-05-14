//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1289/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1289<F: Float>(t16919: F, t28: F, t5778: F, t89: F, t26791: F, t3408: F, t1017: F, t104175: F, t30266: F, t376: F, t17076: F, t5899: F, t5900: F, t9432: F, t105406: F, t1969: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t120029 = t89 * t28 * t5778 * t16919;
    let t120033 = t89 * t28 * t26791 * t3408;
    let t120037 = t89 * t28 * t104175 * t1017;
    let t120040 = t89 * t376 * t30266;
    let t120041 = 4.0 / 3.0 * t120040;
    let t120044 = t5899 * t9432 * t5900 * t17076;
    let t120048 = t5899 * t1969 * t105406 * t925;
    (t120029, t120033, t120037, t120040, t120041, t120044, t120048)
}
