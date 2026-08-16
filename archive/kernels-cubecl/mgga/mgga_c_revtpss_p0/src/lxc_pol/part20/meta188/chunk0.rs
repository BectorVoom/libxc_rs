//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 940/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk940<F: Float>(t3936: F, t3938: F, t9826: F, t3937: F, t4057: F, t5673: F, t1353: F, t4003: F, t4056: F) -> (F, F, F, F, F) {
    let t9828 = t3936 * t9826 * t3938;
    let t9832 = t5673 * t3937 * t4057;
    let t9835 = t4003 * t1353;
    let t9837 = t3936 * t9826 * t9835;
    let t9840 = t4003 * t4056;
    (t9828, t9832, t9835, t9837, t9840)
}
