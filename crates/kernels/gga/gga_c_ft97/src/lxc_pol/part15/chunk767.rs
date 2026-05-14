//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 767/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk767<F: Float>(t13411: F, t4939: F, t17818: F, t7240: F, t81: F, t142: F, t7367: F, t240: F, t7513: F, t294: F, t7639: F, t1107: F, t5011: F, t13: F, t21: F, t2: F, t7242: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30852 = t13411 * t4939;
    let t30853 = t30852 * t17818;
    let t32075 = 1.0 / t7240 / t81;
    let t32905 = 1.0 / t7367 / t142;
    let t33300 = 1.0 / t7513 / t240;
    let t33828 = 1.0 / t7639 / t294;
    let t35382 = t5011 * t1107;
    let t36377 = t13 * t21;
    let t36452 = t7242 * t2;
    (t30852, t30853, t32075, t32905, t33300, t33828, t35382, t36377, t36452)
}
