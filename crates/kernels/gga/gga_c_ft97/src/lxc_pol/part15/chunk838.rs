//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 838/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk838<F: Float>(t21973: F, t2771: F, t21588: F, t848: F, t21204: F, t4206: F, t10594: F, t15011: F, t15025: F, t22302: F, t22306: F, t22310: F, t22313: F, t22316: F, t22319: F, t462: F, t92: F) -> (F, F, F, F) {
    let t22321 = t2771 * t21973;
    let t22323 = t848 * t21588;
    let t22326 = t4206 * t21204;
    let t22329 = -F::new(2.0) * t462 * t22302 - t10594 - t92 * t22306 - F::new(4.0) / F::new(9.0) * t15025 - F::new(4.0) / F::new(3.0) * t15011 + F::new(2.0) / F::new(3.0) * t462 * t22310 + F::new(4.0) / F::new(3.0) * t462 * t22313 - F::new(2.0) / F::new(3.0) * t462 * t22316 + t462 * t22319 + t462 * t22321 - F::new(2.0) * t462 * t22323 + F::new(2.0) * t462 * t22326;
    (t22321, t22323, t22326, t22329)
}
