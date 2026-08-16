//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1187/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1187<F: Float>(t48140: F, t48142: F, t48148: F, t48150: F, t48152: F, t48153: F, t48155: F, t48158: F, t48159: F, t48160: F, t48162: F, t48165: F, t48169: F, t48173: F, t48175: F, t48179: F, t48183: F, t48187: F, t48191: F, t48195: F, t48198: F, t48201: F) -> (F, F) {
    let t48678 = -t48140 + t48142 - t48148 + t48150 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162;
    let t48679 = t48165 + t48169 + t48173 - t48175 - t48179 + t48183 + t48187 - t48191 - t48195 - t48198 - t48201;
    (t48678, t48679)
}
