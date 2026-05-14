//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1043/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1043<F: Float>(t26341: F, t32759: F, t48120: F, t48122: F, t48124: F, t48127: F, t48128: F, t48130: F, t48132: F, t48133: F, t48134: F, t48136: F, t48140: F, t48142: F, t48148: F, t48150: F, t48152: F, t48153: F, t48155: F, t48158: F, t48159: F, t48160: F, t48162: F) -> (F, F) {
    let t48674 = t48120 + t48122 + t48124 + t48127 - t48128 + 8.0 * t32759 - 0.38474813732852776452e0 * t26341 + t48130 + t48132 + t48133 + t48134 + t48136;
    let t48678 = -t48140 + t48142 - t48148 + t48150 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162;
    (t48674, t48678)
}
