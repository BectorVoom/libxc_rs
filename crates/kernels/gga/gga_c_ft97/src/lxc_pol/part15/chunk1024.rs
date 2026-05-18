//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1024/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1024<F: Float>(t4454: F, t4495: F, t1800: F, t24: F, t3127: F, t38508: F, t462: F, t469: F, t58140: F, t59078: F, t59102: F, t59104: F, t59143: F, t74266: F, t74268: F, t74285: F, t74287: F, t8327: F, t85491: F, t85682: F, t85687: F, t85692: F, t92: F) -> (F, F) {
    let t86161 = t4454 * t4495;
    let t86168 = F::new(4.0) / F::new(3.0) * t74266 + F::new(8.0) * t74268 - t92 * t24 * t469 * t85682 + F::new(24.0) * t92 * t24 * t38508 * t85692 + F::new(6.0) * t92 * t24 * t1800 * t85687 - F::new(8.0) / F::new(3.0) * t58140 + F::new(16.0) / F::new(3.0) * t59078 - F::new(8.0) * t74285 - F::new(16.0) / F::new(9.0) * t74287 + F::new(8.0) * t462 * t3127 * t85491 + F::new(4.0) / F::new(3.0) * t462 * t8327 * t86161 + F::new(16.0) / F::new(9.0) * t59102 - F::new(16.0) / F::new(27.0) * t59104 - F::new(8.0) / F::new(9.0) * t59143;
    (t86161, t86168)
}
