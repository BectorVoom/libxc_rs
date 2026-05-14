//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 327/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk327<F: Float>(t1962: F, t143: F, t1557: F, t1559: F, t1555: F, t89: F, t378: F, t525: F) -> (F, F, F, F) {
    let t1963 = t1962 / 9.0;
    let t1964 = t143 * t1557;
    let t1965 = t1964 * t1559;
    let t1967 = t89 * t1555 * t1965;
    let t1969 = t378 * t525;
    (t1963, t1965, t1967, t1969)
}
