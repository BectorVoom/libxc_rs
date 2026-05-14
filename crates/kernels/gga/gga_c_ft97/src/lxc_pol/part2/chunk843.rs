//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 843/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk843<F: Float>(t15128: F, t2844: F, t10688: F, t4181: F, t4239: F, t870: F, t875: F, t2801: F, t4246: F, t10666: F, t1248: F, t2749: F, t4299: F, t15125: F, t295: F, t312: F) -> (F, F, F, F, F, F, F) {
    let t15129 = t15128 * t2844;
    let t15131 = t10688 * t4181;
    let t15133 = t4239 * t870;
    let t15134 = t15133 * t875;
    let t15136 = t4246 * t2801;
    let t15138 = t10666 * t1248;
    let t15140 = t2749 * t4299;
    let t15143 = t295 * t15125 * t312;
    (t15129, t15131, t15134, t15136, t15138, t15140, t15143)
}
