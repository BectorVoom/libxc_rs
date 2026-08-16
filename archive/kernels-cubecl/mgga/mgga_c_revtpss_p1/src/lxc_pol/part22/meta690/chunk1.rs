//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2690/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2690<F: Float>(t22046: F, t3936: F, t9835: F, t1414: F, t21969: F, t828: F, t221: F, t3979: F, t6816: F, t3978: F, t3989: F, t6880: F) -> (F, F, F, F, F) {
    let t22048 = t3936 * t22046 * t9835;
    let t22052 = t1414 * t828 * t21969;
    let t22056 = t3979 * t221 * t6816;
    let t22057 = t3978 * t22056;
    let t22059 = t3989 * t6880;
    (t22048, t22052, t22056, t22057, t22059)
}
