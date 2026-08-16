//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 852/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk852<F: Float>(t4977: F, t679: F, t2378: F, t4939: F, t237: F, t39: F, t13411: F, t17818: F, t7240: F, t81: F, t142: F, t7367: F) -> (F, F, F, F, F, F, F) {
    let t30683 = t679 * t4977;
    let t30688 = t2378 * t4939;
    let t30815 = t237 * t39;
    let t30852 = t13411 * t4939;
    let t30853 = t30852 * t17818;
    let t32075 = F::cast_from(1.0_f64) / t7240 / t81;
    let t32905 = F::cast_from(1.0_f64) / t7367 / t142;
    (t30683, t30688, t30815, t30852, t30853, t32075, t32905)
}
