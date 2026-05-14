//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 560/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk560<F: Float>(t295: F, t312: F, t7091: F, t1248: F, t6353: F, t296: F, t1091: F, t6360: F, t2881: F, t1212: F, t1501: F) -> (F, F, F, F, F, F) {
    let t7093 = t295 * t7091 * t312;
    let t7097 = t6353 * t1248;
    let t7098 = t296 * t7097;
    let t7101 = t6360 * t1091;
    let t7102 = t2881 * t7101;
    let t7105 = t1501 * t1212;
    (t7093, t7097, t7098, t7101, t7102, t7105)
}
