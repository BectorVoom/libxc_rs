//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 506/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk506<F: Float>(t2649: F, t2745: F, t2750: F, t2802: F, t2845: F, t2892: F, t2894: F, t301: F, t317: F, t830: F, t880: F, t332: F) -> (F, F) {
    let t2899 = -t2649 * t317 - t2745 * t317 - t2892 * t301 - F::cast_from(2.0_f64) * t830 * t880 - F::cast_from(4.0_f64) * t2750 - F::cast_from(2.0_f64) * t2802 + F::cast_from(4.0_f64) * t2845 + F::cast_from(2.0_f64) * t2894;
    let t2900 = t2899 * t332;
    (t2899, t2900)
}
