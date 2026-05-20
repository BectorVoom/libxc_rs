//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3936/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3936<F: Float>(t22571: F, t571: F, t1458: F, t18178: F, t18217: F, t1914: F, t1921: F, t4168: F, t5790: F, t5808: F, t60609: F, t60611: F, t60616: F, t60618: F, t6937: F, t75727: F, t75760: F, t75792: F) -> F {
    let t75796 = t571 * t22571;
    let t75801 = F::new(2.0) * t60609 + F::new(2.0) * t18178 * t1921 + F::new(2.0) * t60611 + F::new(2.0) * t75727 + F::new(4.0) * t60616 + F::new(4.0) * t5790 * t5808 + t1458 * (t75760 + t75792) + F::new(2.0) * t60618 + F::new(2.0) * t75796 + t6937 * t4168 + F::new(2.0) * t1914 * t18217;
    t75801
}
