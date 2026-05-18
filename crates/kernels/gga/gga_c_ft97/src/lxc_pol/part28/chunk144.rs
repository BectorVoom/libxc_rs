//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 144/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk144<F: Float>(t167: F, t558: F, t574: F, t153: F, t151: F, t458: F, t143: F, t355: F) -> (F, F, F, F) {
    let t576 = t574 * t167 * t558;
    let t579 = F::new(1.0) / t153;
    let t581 = t458 * t151 / F::new(3.0);
    let t582 = t355 * t143;
    (t576, t579, t581, t582)
}
