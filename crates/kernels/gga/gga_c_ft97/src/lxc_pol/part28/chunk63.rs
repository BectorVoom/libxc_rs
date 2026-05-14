//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 63/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk63<F: Float>(t25: F, t31: F, t120: F) -> (F, F, F, F) {
    let t122 = t25 * t25;
    let t123 = t122 * t25;
    let t126 = f64::exp(-0.16390970575e0 * t123 * t31);
    let t128 = 0.1247511874e1 - 0.859614445e0 * t120 + 0.812904345e0 * t126;
    (t122, t123, t126, t128)
}
