//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1318/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1318<F: Float>(t241: F, t57275: F, t57338: F, t57383: F, t57517: F, t57233: F, t57236: F, t57238: F, t57240: F, t57244: F, t57246: F, t57248: F, t57251: F, t57253: F, t57257: F, t57260: F) -> (F, F) {
    let t57520 = t241 * (t57275 + t57338 + t57383 + t57517);
    let t57521 = -t57233 - t57236 - t57238 - t57240 - t57244 - t57246 - t57248 - t57251 + t57253 + t57257 + t57260 + t57520;
    (t57520, t57521)
}
