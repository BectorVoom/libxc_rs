//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1718/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1718<F: Float>(t124: F, t1398: F, t3938: F, t9818: F, t9816: F, t1353: F, t4003: F, t4056: F) -> (F, F, F, F) {
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9835 = t4003 * t1353;
    let t9840 = t4003 * t4056;
    (t9821, t9822, t9835, t9840)
}
