//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1005/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1005<F: Float>(t1006: F, t12590: F, t3392: F, t3493: F, t41595: F, t41633: F, t10629: F, t3500: F, t12616: F, t5211: F, t7106: F, t41666: F, t41668: F, t12797: F, t2615: F, t2559: F, t47446: F, t587: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t48148 = 16.0 / 5.0 * t1006 * t12590;
    let t48150 = 16.0 / 5.0 * t3493 * t3392;
    let t48152 = 32.0 / 15.0 * t41595;
    let t48153 = 64.0 / 45.0 * t41633;
    let t48155 = 32.0 / 15.0 * t10629 * t3500;
    let t48158 = 32.0 / 15.0 * t5211 * t7106 * t12616;
    let t48159 = 32.0 / 45.0 * t41666;
    let t48160 = 64.0 / 45.0 * t41668;
    let t48162 = 16.0 / 9.0 * t2615 * t12797;
    let t48165 = 16.0 / 27.0 * t587 * t2559 * t47446;
    (t48148, t48150, t48152, t48153, t48155, t48158, t48159, t48160, t48162, t48165)
}
