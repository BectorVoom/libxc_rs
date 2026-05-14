//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1006/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1006<F: Float>(t34544: F, t48148: F, t48150: F, t48152: F, t48153: F, t48155: F, t48158: F, t48159: F, t48160: F, t48162: F, t48165: F, t47450: F, t587: F, t7435: F, t12460: F, t1820: F, t995: F) -> (F, F, F) {
    let t48166 = -t48148 + t48150 + 0.72933333333333333331e0 * t34544 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162 + t48165;
    let t48169 = 64.0 / 27.0 * t587 * t7435 * t47450;
    let t48173 = 256.0 / 81.0 * t1820 * t7435 * t12460 * t995;
    (t48166, t48169, t48173)
}
