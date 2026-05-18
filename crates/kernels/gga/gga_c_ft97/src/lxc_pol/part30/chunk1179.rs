//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1179/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1179<F: Float>(t25462: F, t35810: F, t143007: F, t143008: F, t143018: F, t143024: F, t1466: F, t154083: F, t154463: F, t154503: F, t154550: F, t154851: F, t193: F, t28835: F, t29416: F, t34056: F, t36093: F, t6225: F, t7618: F) -> F {
    let t155010 = t25462 * t35810;
    let t155028 = t143007 + t155010 / F::new(54.0) - t143008 / F::new(9.0) - F::new(2.0) * t154083 + t29416 * t7618 / F::new(3.0) + t143018 / F::new(54.0) - t143024 / F::new(27.0) - F::new(2.0) * t154550 + F::new(4.0) * t154463 - F::new(4.0) * t154503 - F::new(2.0) / F::new(3.0) * t1466 * t193 * t28835 * t34056 - t36093 * t6225 / F::new(3.0) - F::new(2.0) * t154851;
    t155028
}
