//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1012/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1012<F: Float>(t32399: F, t6414: F, t137356: F, t137363: F, t144704: F, t144708: F, t144714: F, t144719: F, t144725: F, t144727: F, t1564: F, t25530: F, t25558: F, t32013: F, t32021: F, t379: F, t5501: F, t5504: F, t7162: F) -> F {
    let t144729 = t6414 * t32399;
    let t144731 = -F::new(4.0) * t144704 - t137356 / F::new(9.0) - t137363 / F::new(9.0) + t144708 / F::new(54.0) + t25558 * t32021 / F::new(9.0) - t25558 * t32013 / F::new(18.0) - t5501 * t1564 * t144714 * t379 / F::new(18.0) - t144719 * t5504 / F::new(18.0) - t7162 * t25530 / F::new(3.0) - t144725 / F::new(18.0) - t144727 / F::new(18.0) + t144729 / F::new(9.0);
    t144731
}
