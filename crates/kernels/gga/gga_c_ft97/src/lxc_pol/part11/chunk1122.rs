//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1122/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1122<F: Float>(t41448: F, t43495: F, t89: F, t9716: F, t193: F, t2682: F, t2739: F, t7640: F, t2675: F, t9733: F, t43453: F, t43457: F, t43460: F, t43463: F, t43466: F, t43471: F, t43474: F, t43478: F, t43483: F, t43487: F, t43490: F, t43493: F) -> (F, F, F, F) {
    let t43498 = t89 * t9716 * t43495 * t41448;
    let t43503 = t89 * t193 * t7640 * t2682 * t2739;
    let t43506 = t89 * t9733 * t2675;
    let t43508 = -F::new(8.0) / F::new(3.0) * t43453 - F::new(8.0) / F::new(3.0) * t43457 + F::new(4.0) / F::new(27.0) * t43460 + F::new(16.0) / F::new(27.0) * t43463 + F::new(8.0) / F::new(9.0) * t43466 + F::new(40.0) / F::new(81.0) * t43471 - F::new(20.0) / F::new(27.0) * t43474 + F::new(8.0) / F::new(3.0) * t43478 - F::new(80.0) / F::new(243.0) * t43483 - t43487 / F::new(9.0) - F::new(16.0) / F::new(27.0) * t43490 + F::new(40.0) / F::new(243.0) * t43493 + F::new(40.0) / F::new(27.0) * t43498 - F::new(12.0) * t43503 - F::new(8.0) / F::new(27.0) * t43506;
    (t43498, t43503, t43506, t43508)
}
