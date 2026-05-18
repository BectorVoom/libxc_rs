//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1140/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1140<F: Float>(t1006: F, t12590: F, t3392: F, t3493: F, t41595: F, t41633: F, t10629: F, t3500: F, t12616: F, t5211: F, t7106: F, t41666: F) -> (F, F, F, F, F, F, F) {
    let t48148 = F::new(16.0) / F::new(5.0) * t1006 * t12590;
    let t48150 = F::new(16.0) / F::new(5.0) * t3493 * t3392;
    let t48152 = F::new(32.0) / F::new(15.0) * t41595;
    let t48153 = F::new(64.0) / F::new(45.0) * t41633;
    let t48155 = F::new(32.0) / F::new(15.0) * t10629 * t3500;
    let t48158 = F::new(32.0) / F::new(15.0) * t5211 * t7106 * t12616;
    let t48159 = F::new(32.0) / F::new(45.0) * t41666;
    (t48148, t48150, t48152, t48153, t48155, t48158, t48159)
}
