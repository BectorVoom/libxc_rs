//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1141/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1141<F: Float>(t41668: F, t12797: F, t2615: F, t2559: F, t47446: F, t587: F, t34544: F, t48148: F, t48150: F, t48152: F, t48153: F, t48155: F, t48158: F, t48159: F) -> (F, F, F, F) {
    let t48160 = F::new(64.0) / F::new(45.0) * t41668;
    let t48162 = F::new(16.0) / F::new(9.0) * t2615 * t12797;
    let t48165 = F::new(16.0) / F::new(27.0) * t587 * t2559 * t47446;
    let t48166 = -t48148 + t48150 + F::new(0.72933333333333333331e0) * t34544 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162 + t48165;
    (t48160, t48162, t48165, t48166)
}
