//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1148/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1148<F: Float>(t21531: F, t51340: F, t1091: F, t14159: F, t18467: F, t1901: F, t21646: F, t21673: F, t21753: F, t2599: F, t2606: F, t3892: F, t42517: F, t51990: F, t80334: F, t80460: F, t81365: F, t81411: F, t81448: F, t81454: F, t81469: F, t89222: F) -> (F, F) {
    let t89371 = t51340 * t21531;
    let t89404 = -F::new(8.0) / F::new(3.0) * t81365 - F::new(8.0) / F::new(9.0) * t81411 + F::new(8.0) / F::new(3.0) * t1901 * t2599 * t3892 * t89222 + F::new(4.0) / F::new(3.0) * t1901 * t14159 * t21646 + F::new(8.0) / F::new(3.0) * t1901 * t42517 * t80460 * t1091 + F::new(8.0) / F::new(9.0) * t1901 * t18467 * t21753 + F::new(8.0) / F::new(9.0) * t1901 * t51990 * t21673 + F::new(4.0) / F::new(9.0) * t1901 * t2606 * t80334 * t1091 - F::new(8.0) / F::new(9.0) * t81448 + F::new(4.0) / F::new(9.0) * t81454 - F::new(4.0) / F::new(9.0) * t81469;
    (t89371, t89404)
}
