//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 977/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk977<F: Float>(t10666: F, t1248: F, t2749: F, t4299: F, t15125: F, t295: F, t312: F, t1250: F, t8232: F, t1091: F, t2894: F, t835: F) -> (F, F, F, F, F) {
    let t15138 = t10666 * t1248;
    let t15140 = t2749 * t4299;
    let t15143 = t295 * t15125 * t312;
    let t15147 = t8232 * t1250;
    let t15150 = t835 * t2894 * t1091;
    (t15138, t15140, t15143, t15147, t15150)
}
