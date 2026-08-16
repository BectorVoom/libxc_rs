//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 985/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk985<F: Float>(t22330: F, t2755: F, t1882: F, t21946: F, t21982: F, t681: F, t89: F, t21989: F, t2336: F, t21962: F, t9725: F, t21974: F) -> (F, F, F, F, F, F) {
    let t83615 = t2755 * t22330;
    let t83619 = t1882 * t21946;
    let t83652 = t89 * t681 * t21982;
    let t83655 = t89 * t2336 * t21989;
    let t83683 = t89 * t9725 * t21962;
    let t83718 = t1882 * t21974;
    (t83615, t83619, t83652, t83655, t83683, t83718)
}
