//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 611/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk611<F: Float>(t1557: F, t8654: F, t1736: F, t179: F, t1068: F, t8640: F, t171: F, t7741: F, t1075: F, t7773: F, t89: F, t998: F) -> (F, F, F, F, F, F) {
    let t12122 = t8654 * t1557;
    let t12137 = t1736 * t179;
    let t12165 = t8640 * t1068;
    let t12168 = F::new(1.0) / t171 / t7741;
    let t12204 = t8640 * t1075;
    let t12362 = t89 * t7773 * t998;
    (t12122, t12137, t12165, t12168, t12204, t12362)
}
