//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 865/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk865<F: Float>(t15199: F, t15252: F, t15307: F, t15341: F, t15401: F, t15453: F, t15496: F, t15543: F, t14911: F, t14914: F, t15074: F, t15126: F, t15129: F, t15131: F, t15134: F, t15136: F, t15138: F, t15140: F, t301: F, t317: F) -> (F,) {
    let t15546 = t15199 + t15252 + t15307 + t15341 + t15401 + t15453 + t15496 + t15543;
    let t15548 = -2.0 * t14911 * t317 - t14914 * t317 - t15546 * t301 - 2.0 * t15074 + 2.0 * t15126 + 4.0 * t15129 + 8.0 * t15131 - 4.0 * t15134 - 2.0 * t15136 - 2.0 * t15138 - 4.0 * t15140;
    (t15548,)
}
